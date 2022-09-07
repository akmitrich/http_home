use actix_web::{web, HttpRequest, HttpResponse, Responder};
use querystring::querify;
use std::collections::HashMap;
use std::num::ParseFloatError;
use thiserror::Error;
use crate::smart_device::Device;
use crate::SmartHome;

#[derive(Debug, Error)]
pub enum HandleRequestError {
    #[error("Device not found {0}.")]
    DeviceNotFound(String),
    #[error("Error in device dict {0}.")]
    BadDeviceDict(String),
    #[error("Parse float error {0}.")]
    ParseFloatError(#[from] ParseFloatError),
}

type HandleRequestResult<T> = Result<T, HandleRequestError>;

pub async fn greet(_: HttpRequest, home: web::Data<SmartHome>) -> impl Responder {
    let home = home.read().await;
    format!("Otus Smart Home Server is here.\n Home name is {}", home.get_name())
}

pub async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn room_list(_: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let home = home.read().await;
    let room_list: Vec<_> = home.room_names_list().collect();
    HttpResponse::Ok().json(room_list)
}

pub async fn device_list(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = req.match_info().get("room_name").unwrap_or("");
    let home = home.read().await;
    match home.device_names_list(room_name) {
        Some(list) => HttpResponse::Ok().json(list),
        None => HttpResponse::BadRequest().body(format!("Room not found {}", room_name)),
    }
}

pub async fn update(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = dbg!(req.match_info().get("room_name").unwrap_or_default());
    let device_name = dbg!(req.match_info().get("device_name").unwrap_or_default());
    let info = req.query_string();
    let data: HashMap<_, _> = querify(info).into_iter().collect();
    match dbg!(update_device(home, room_name, device_name, data).await) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            HandleRequestError::DeviceNotFound(_) => HttpResponse::BadRequest().body(format!("{}", e)),
            HandleRequestError::BadDeviceDict(_) => HttpResponse::BadRequest().body(format!("{}", e)),
            HandleRequestError::ParseFloatError(_) => HttpResponse::BadRequest().body(format!("{}", e)),
//            _ => HttpResponse::InternalServerError().body(format!("{}", e)),
        }
    }
}

async fn update_device(
    home: web::Data<SmartHome>,
    room_name: &str,
    device_name: &str,
    data: HashMap<&str, &str>,
) -> HandleRequestResult<()> {
    let device_string = data
        .get("device")
        .ok_or_else(|| HandleRequestError::BadDeviceDict(format!("{:#?}", data)))?
        .to_lowercase();
    let mut home = home.write().await;
    let device = home
        .get_device_by_path_mut(room_name, device_name)
        .ok_or_else(|| {
            HandleRequestError::DeviceNotFound(format!("{}/{}", room_name, device_name))
        })?;
    match device {
        Device::Socket(socket) => {
            if device_string == "socket" {
                if let Some(state) = data.get("state") {
                    match state.to_lowercase().as_str() {
                        "on" | "вкл" => socket.switch(true),
                        "off" | "выкл" => socket.switch(false),
                        _ => return Err(HandleRequestError::BadDeviceDict(format!("{:#?}", data))),
                    }
                }
                if let Some(current) = data.get("current") {
                    socket.set_current(current.parse()?);
                }
                if let Some(voltage) = data.get("voltage") {
                    socket.set_voltage(voltage.parse()?);
                }
            } else {
                return Err(HandleRequestError::BadDeviceDict(format!("{:#?}", data)));
            }
        }
        Device::Thermometer(thermometer) => {
            if device_string == "thermometer" {
                if let Some(temperature) = data.get("temperature") {
                    thermometer.set_temperature(temperature.parse()?);
                }
            } else {
                return Err(HandleRequestError::BadDeviceDict(format!("{:#?}", data)));
            }
        }
        _ => todo!(),
    }
    Ok(())
}
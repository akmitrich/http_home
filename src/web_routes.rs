use actix_web::{web, HttpRequest, HttpResponse, Responder};
use querystring::querify;
use std::collections::HashMap;
use std::num::ParseFloatError;
use thiserror::Error;
use crate::smart_device::{Device, Socket, Thermometer, DeviceDict};
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

pub async fn add_room(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    if let Some(room_name) = req.match_info().get("room_name") {
        let mut home = home.write().await;
        match home.add_room(room_name) {
            Some(_) => HttpResponse::Ok().finish(),
            None => HttpResponse::BadRequest().body(format!("Cannot add room '{}' because it already exists.", room_name)),
        }
    } else { // this code must be unreachable 
        HttpResponse::InternalServerError().body(format!("Unexpected request: '{:#?}'", req))
    }
}

pub async fn add_device(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = req.match_info().get("room_name").unwrap_or_default();
    if let Some(device_name) = req.match_info().get("device_name") {
        let mut home = home.write().await;
        let query_string = req.query_string();
        let data: HashMap<_, _> = querify(query_string).into_iter().collect();
        match create_device_from(data) {
            Ok(new_device) => {
                match home.add_device(room_name, device_name, new_device) {
                    Some(_) => HttpResponse::Ok().finish(),
                    None => HttpResponse::BadRequest().body(format!("Room not found '{}' or duplicate device '{}'.", room_name, device_name)),
                }
            }
            Err(e) => match e {
                HandleRequestError::BadDeviceDict(_) | HandleRequestError::ParseFloatError(_) => HttpResponse::BadRequest().body(format!("{}", e)),
                _ => HttpResponse::InternalServerError().body(format!("Unexpected request: '{:#?}'", req)),
            }
        }
    } else { // this code must be unreachable 
        HttpResponse::InternalServerError().body(format!("Unexpected request: '{:#?}'", req))
    }
}

pub async fn remove_device(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = req.match_info().get("room_name").unwrap_or_default();
    let device_name = req.match_info().get("device_name").unwrap_or_default();
    let mut home = home.write().await;
    match home.remove_device(room_name, device_name) {
        Some(device) => HttpResponse::Ok().json(device.device_dict()),
        None => HttpResponse::BadRequest().body(format!("Device not found for room/device '{room_name}/{device_name}'.")),
    }
}

pub async fn remove_room(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = req.match_info().get("room_name").unwrap_or_default();
    let mut home = home.write().await;
    match home.remove_room(room_name) {
        Some(_) => HttpResponse::Ok().body(format!("Removed room '{room_name}'.")),
        None => HttpResponse::BadRequest().body(format!("Room not found '{room_name}'.")),
    }
}

pub async fn update(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = dbg!(req.match_info().get("room_name").unwrap_or_default());
    let device_name = dbg!(req.match_info().get("device_name").unwrap_or_default());
    let query_string = req.query_string();
    let data: HashMap<_, _> = querify(query_string).into_iter().collect();
    match dbg!(update_device(home, room_name, device_name, data).await) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
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
                        _ => return Err(bad_device_dict_error(&data)),
                    }
                }
                if let Some(current) = data.get("current") {
                    socket.set_current(current.parse()?);
                }
                if let Some(voltage) = data.get("voltage") {
                    socket.set_voltage(voltage.parse()?);
                }
            } else {
                return Err(bad_device_dict_error(&data));
            }
        }
        Device::Thermometer(thermometer) => {
            if device_string == "thermometer" {
                if let Some(temperature) = data.get("temperature") {
                    thermometer.set_temperature(temperature.parse()?);
                }
            } else {
                return Err(bad_device_dict_error(&data));
            }
        }
        _ => todo!(),
    }
    Ok(())
}

fn create_device_from(data: HashMap<&str, &str>) -> HandleRequestResult<Device> {
    let device_string = data.get("device").ok_or_else( || bad_device_dict_error(&data))?;
    match device_string.to_lowercase().as_str() {
        "socket" => {
            let state = data.get("state").ok_or_else(|| bad_device_dict_error(&data))?;
            let on = match state.to_lowercase().as_str() {
                "on" | "вкл" => Some(true),
                "off" | "выкл" => Some(false),
                _ => None,
            }.ok_or_else(|| bad_device_dict_error(&data))?;
            let voltage = data.get("voltage")
                .ok_or_else(|| bad_device_dict_error(&data))?
                .parse::<f64>()?;
            let current = data.get("current")
                .ok_or_else(|| bad_device_dict_error(&data))?
                .parse::<f64>()?;
            Ok(Device::Socket(Socket::new(voltage, current, on)))
        }
        "thermometer" => {
            let temperature = data.get("temperature")
                .ok_or_else(|| bad_device_dict_error(&data))?
                .parse::<f64>()?;
            Ok(Device::Thermometer(Thermometer::new(temperature)))
        }
        _ => Err(bad_device_dict_error(&data))
    }
}

fn bad_device_dict_error(data: &HashMap<&str, &str>) -> HandleRequestError {
    HandleRequestError::BadDeviceDict(format!("{:#?}", data))
}
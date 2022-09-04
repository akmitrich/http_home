use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use querystring::querify;
use smart_device::Device;
use std::collections::HashMap;
use std::net::TcpListener;
use std::num::ParseFloatError;
use thiserror::Error;
use tokio::sync::RwLock;

pub mod home;
pub mod smart_device;
pub mod smart_room;

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

async fn greet(_: HttpRequest) -> impl Responder {
    "Otus Smart Home Server is here.\n "
}

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

type SmartHome = RwLock<home::Home>;

async fn update(req: HttpRequest, home: web::Data<SmartHome>) -> HttpResponse {
    let room_name = dbg!(req.match_info().get("room_name").unwrap_or_default());
    let device_name = dbg!(req.match_info().get("device_name").unwrap_or_default());
    let info = req.query_string();
    let data: HashMap<_, _> = querify(info).into_iter().collect();
    match dbg!(update_device(home, room_name, device_name, data).await) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
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
    match dbg!(home.get_device_by_path_mut(room_name, device_name)) {
        Some(device) => {
            match device {
                Device::Socket(socket) => {
                    if device_string == "socket" {
                        if let Some(state) = data.get("state") {
                            match state.to_lowercase().as_str() {
                                "on" | "вкл" => socket.switch(true),
                                "off" | "выкл" => socket.switch(false),
                                _ => {
                                    return Err(HandleRequestError::BadDeviceDict(format!(
                                        "{:#?}",
                                        data
                                    )))
                                }
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
        None => Err(HandleRequestError::DeviceNotFound(format!(
            "{}/{}",
            room_name, device_name
        ))),
    }
}

pub fn run(listener: TcpListener, home: home::Home) -> std::io::Result<Server> {
    let smart_home = web::Data::new(SmartHome::new(home));
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/update/{room_name}/{device_name}", web::post().to(update))
            .route("/{name}", web::get().to(greet))
            .app_data(web::Data::clone(&smart_home))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

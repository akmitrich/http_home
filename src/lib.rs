use actix_web::{App, HttpServer, web, HttpRequest, Responder, HttpResponse};
use actix_web::dev::Server;
use tokio::sync::RwLock;
use std::net::TcpListener;

pub mod home;
pub mod smart_room;
pub mod smart_device;

async fn greet(_: HttpRequest) -> impl Responder {
    "Otus Smart Home Server is here.\n "
}

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct DeviceForm {
    room_name: String,
    device_name: String,
    data: String,
}

type SmartHome = RwLock<home::Home>;

async fn update(form: web::Form<DeviceForm>, home: web::Data<SmartHome>) -> HttpResponse {
    let mut home = home.read().await;
    println!("Update Home: {:?}", home.room_names_list().collect::<Vec<&String>>());
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener, home: home::Home) -> std::io::Result<Server> {
    let smart_home = web::Data::new(SmartHome::new(home));
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/update", web::post().to(update))
            .route("/{name}", web::get().to(greet))
            .app_data(web::Data::clone(&smart_home))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
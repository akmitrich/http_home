use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tokio::sync::RwLock;

pub mod home;
pub mod smart_device;
pub mod smart_room;
pub mod web_routes;

type SmartHome = RwLock<home::Home>;

pub fn run(listener: TcpListener, home: home::Home) -> std::io::Result<Server> {
    let smart_home = web::Data::new(SmartHome::new(home));
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(web_routes::greet))
            .route("/health_check", web::get().to(web_routes::health_check))
            .route("/room_list", web::get().to(web_routes::room_list))
            .route(
                "/device_list/{room_name}",
                web::get().to(web_routes::device_list),
            )
            .route("add_room/{room_name}", web::post().to(web_routes::add_room))
            .route(
                "add_device/{room_name}/{device_name}",
                web::post().to(web_routes::add_device),
            )
            .route(
                "/remove_device/{room_name}/{device_name}",
                web::post().to(web_routes::remove_device),
            )
            .route(
                "remove_room/{room_name}",
                web::post().to(web_routes::remove_room),
            )
            .route(
                "/update/{room_name}/{device_name}",
                web::post().to(web_routes::update),
            )
            .route("/report", web::get().to(web_routes::report))
            .route("/{room_name}/{device_name}", web::get().to(web_routes::get_device))
            .app_data(web::Data::clone(&smart_home))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

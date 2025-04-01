use controllers::{informacoes_classes::lista_classes, informacoes_origens::lista_origens};
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

pub mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![lista_origens])
    .mount("/", routes![lista_classes])
    .attach(Cors::from_options(&CorsOptions::default()
    .allowed_origins(AllowedOrigins::All)).unwrap())
    .configure(rocket::Config::figment().merge(("port", 3060)))
}
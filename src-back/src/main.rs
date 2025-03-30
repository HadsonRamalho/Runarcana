use controllers::informacoes_origens::lista_origens;
use rocket::{get, launch, routes};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

pub mod controllers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![lista_origens])
    .attach(Cors::from_options(&CorsOptions::default()
        .allowed_origins(AllowedOrigins::All)).unwrap())
}
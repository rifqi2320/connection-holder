mod controller;
mod guard;

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use controller::state::Clients;
use controller::{publish, subscribe, healthcheck};
use guard::KeyGuardAuthorization;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let clients: Clients = Arc::new(Mutex::new(Default::default()));

    println!("Server running at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(clients.clone()))
            .service(healthcheck)
            .service(subscribe)
            .service(
                web::scope("/private")
                    .guard(KeyGuardAuthorization)
                    .service(publish),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

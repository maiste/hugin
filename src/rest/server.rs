/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v. 2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[actix_web::main]
pub async fn run(port: String) -> std::io::Result<()> {
    let addr = format!("localhost:{}", port);
    HttpServer::new(|| App::new().service(ping))
        .bind(addr)?
        .run()
        .await
}

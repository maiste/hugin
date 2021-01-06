/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v. 2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use actix_web::{body::Body, get, guard, web, App, HttpResponse, HttpServer, Responder};

#[get("/test")]
pub async fn test() -> impl Responder {
    let content = String::from("{ \"test\":\"OK\"}");
    let body = Body::from(content);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/{name}")]
async fn json_loader(web::Path(name): web::Path<String>) -> impl Responder {
    let content = String::from(format!("{{ \"name\" : \"{}\" }}", name));
    let body = Body::from(content);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[actix_web::main]
pub async fn run(port: String) -> std::io::Result<()> {
    let addr = format!("localhost:{}", port);
    HttpServer::new(|| {
        App::new()
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::NotFound()),
            )
            .service(test)
            .service(json_loader)
    })
    .bind(addr)?
    .run()
    .await
}

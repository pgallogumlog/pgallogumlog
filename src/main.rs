use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpServer, Responder};

#[derive(Deserialize, Serialize)]
struct EchoRequest {
    message: String,
}

#[derive(Serialize)]
struct EchoResponse {
    message: String
}

async fn echo(req_body: web::Json<EchoRequest>) -> impl Responder {
    let body = req_body.into_inner();
    let res = EchoResponse {
        message: body.message
    };

    web::Json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/echo", web::post().to(echo))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

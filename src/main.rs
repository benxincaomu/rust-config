use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Once;
use log::{error, info, warn};
use log4rs;
fn init() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
}


#[get("/")]
async fn index() -> impl Responder {
    info!("index request");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    static ONCE: Once = Once::new();
    ONCE.call_once(init);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

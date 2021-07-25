use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index_get() -> impl Responder {
  HttpResponse::Ok().body("index page")
}

#[post("/echo")]
async fn echo_post(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new( || {
      App::new()
	.service(index_get)
	.service(echo_post)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}

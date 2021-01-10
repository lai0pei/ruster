use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn get() -> impl Responder{
    "<div><h1>Hello get</h1></div>"
}

async fn home() -> impl Responder{
    "<div><h1>Hello home</h1></div>"
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
   move || App::new().service(web::scope("/app").service(web::resource("/home/").route(web::get().to(|| home() ))))
   .service(get)
                 .default_service(web::route().to(|| HttpResponse::Found()))).workers(2)
        .bind("localhost:8080")?
        .run()
        .await
}
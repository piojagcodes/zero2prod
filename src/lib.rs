use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;


async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run() -> Result<Server, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let local_address = listener.local_addr()?;

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    println!("Server running at: http://{}", local_address);
    Ok(server)
}

use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/js", "static/js/").show_files_listing())
            .service(Files::new("/img", "static/img/").show_files_listing())
            .service(Files::new("/css", "static/css/").show_files_listing())
            .service(Files::new("/", "./static/html").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

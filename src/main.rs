use actix_files::Files;
use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(Files::new("/js", "static/js/").show_files_listing())
            .service(Files::new("/img", "static/img/").show_files_listing())
            .service(Files::new("/css", "static/css/").show_files_listing())
            .service(Files::new("/", "./static/html").index_file("index.html"));
    };

    Ok(config.into())
}

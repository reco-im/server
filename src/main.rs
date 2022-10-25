mod api;
mod model;
mod repository;

use api::task::{
    get_task,
    submit_task,
    start_task,
    complete_task,
    pause_task,
    fail_task,
};
use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    HttpServer::new(move || {
        // 매 thread 마다 새로 쓰도록
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("task"), // table name
            config.clone()
        );

        let ddb_data = Data::new(
            ddb_repo
        );

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(submit_task)
            .service(start_task)
            .service(complete_task)
            .service(pause_task)
            .service(fail_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
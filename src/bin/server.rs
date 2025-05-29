// src/bin/server.rs
use actix_web::{middleware, web, HttpServer, App};
use blog_ddd::app;
use blog_ddd::config::db::connection::establish_connection;
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use blog_ddd::infrastructure::swagger::swagger_docs::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    log::info!("starting HTTP server at http://{}:{}", host, port);
    //println!("Server running at http://{}:{}", host, port);

    let pool = establish_connection().await;
    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(pool_data.clone())
            .configure(app)
            .service(
                SwaggerUi::new("/docs-v1/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}


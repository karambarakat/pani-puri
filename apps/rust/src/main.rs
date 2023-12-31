use actix_web::{web, App, HttpResponse, HttpServer};
use async_graphql::{EmptySubscription, Schema};

mod db;
mod errors;
mod graphql;
mod middlewares;
mod modules;
mod services;
mod utils;

const HI: &str = "new str";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("port is not a number");

    println!("connecting to db");

    let pool = db::connect().await;

    println!("connected to db, binding on port {port}");

    HttpServer::new(move || {
        let schema = Schema::build(
            crate::graphql::root::Query::default(),
            crate::graphql::root::Mutation::default(),
            EmptySubscription,
        )
        .data(pool.clone())
        .finish();

        App::new()
            .wrap(crate::middlewares::respond::Middleware)
            .wrap(crate::middlewares::user::Middleware)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/graphql")
                    .app_data(web::Data::new(schema))
                    .wrap(crate::middlewares::bearer_token::Middleware)
                    .route(web::post().to(crate::graphql::graphql_endpoint)),
            )
            .service(
                web::scope("/auth/local")
                    .wrap(crate::middlewares::basic_token::Middleware)
                    .configure(crate::services::local_auth::config),
            )
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("api is working") }),
            )
            .default_service(web::to(|| async { crate::errors::MyErrors::NotFound }))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
    .map_err(|val| {
        println!("app is not running");
        val
    })
}

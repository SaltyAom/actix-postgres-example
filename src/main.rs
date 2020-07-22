#[macro_use]
extern crate diesel;

use std::io;

use actix_web::{ HttpServer, App };

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

mod model;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let connection_url = std::env::var("DATABASE_URL").expect("Database Url");
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::index)
            .service(routes::get)
            .service(routes::insert)
    })
    .bind("127.0.0.1:80")?
    .run()
    .await
}
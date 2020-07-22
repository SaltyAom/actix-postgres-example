use actix_web::{ get, web, HttpResponse, Error };

use crate::model::User;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
pub async fn index() -> String {
    "Hello World".to_owned()
}

#[get("/insert/{username}/{password}")]
pub async fn insert(param: web::Path<(String, String)>, connection_pool: web::Data<ConnectionPool>) -> Result<HttpResponse, Error> {
    let username = param.0.to_owned();
    let password = param.1.to_owned();

    let connection = connection_pool.get().expect("Couldn't get database connection");

    let user = web::block(move || {
        let _user = User {
            username: username,
            password: password
        };

        _user.insert(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/get/{username}/{password}")]
pub async fn get(param: web::Path<(String, String)>, connection_pool: web::Data<ConnectionPool>) -> Result<HttpResponse, Error> {
    let username = param.0.to_owned();
    let password = param.1.to_owned();

    let connection = connection_pool.get().expect("Couldn't get database connection");

    let user = web::block(move || {
        let _user = User {
            username: username,
            password: password
        };

        _user.get(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().body("Username or Password is incorrect".to_owned()))
    }
}
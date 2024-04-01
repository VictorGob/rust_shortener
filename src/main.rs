#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use url::Url;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use sqlx::Error;

#[derive(Database)]
#[database("local_url_shortener")]
struct Urls(sqlx::PgPool);

#[post("/", data = "<url>")]
async fn short_creation(mut db: Connection<Urls>, url: &str) -> Result<String, Status> {
    // TODO: Retrieve port from config
    let p_url = Url::parse(url).map_err(|_| Status::UnprocessableEntity)?;

    let id_result = sqlx::query("SELECT id FROM urls_data WHERE url = $1;")
        .bind(&p_url.to_string())
        .fetch_one(&mut **db)
        .await;

    let id = match id_result {
        Ok(row) => match row.try_get::<String, _>(0) {
            Ok(id) => return Ok(format!("http://localhost:8001/{}", id.trim())),
            Err(_) => nanoid::nanoid!(6),
        },
        Err(Error::RowNotFound) => nanoid::nanoid!(6),
        Err(e) => {
            println!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    println!("Generated ID: {}", id);
    let _ = sqlx::query("INSERT INTO urls_data (id, url) VALUES ($1, $2);")
        .bind(id.trim())
        .bind(&p_url.to_string())
        .execute(&mut **db)
        .await
        .map_err(|e| {
            println!("Error inserting data: {}", e);
            Status::InternalServerError
        });

    Ok(format!("http://localhost:8001/{}", id.trim()))
}

#[get("/")]
fn landing() -> String {
    // TODO: Add a landing page
    "Hello, world!".to_string()
}

#[get("/<url_id>")]
async fn shortener(mut db: Connection<Urls>, url_id: &str) -> Result<Redirect, Status> {
    let id_result = sqlx::query("SELECT url FROM urls_data WHERE id = $1;")
        .bind(url_id)
        .fetch_one(&mut **db)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => Status::NotFound,
            _ => Status::InternalServerError,
        });
    let id = match id_result {
        Ok(row) => match row.try_get::<String, _>(0) {
            Ok(id) => id,
            Err(_) => {
                println!("Error: {}", Error::ColumnNotFound("id".to_string()));
                return Err(Status::InternalServerError);
            }
        },
        Err(e) => {
            println!("Error retrieving url data: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    println!("Redirecting to: {}", id);
    Ok(Redirect::to(id))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Urls::init())
        .mount("/", routes![shortener, landing, short_creation])
}

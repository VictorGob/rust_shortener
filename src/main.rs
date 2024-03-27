#[macro_use]
extern crate rocket;

use rocket::http::Status;
use url::Url;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use sqlx::Error;

#[derive(Database)]
#[database("local_url_shortener")]
struct Urls(sqlx::PgPool);

#[post("/", data = "<url>")]
async fn short_creation(mut db: Connection<Urls>, url: &str) -> Result<String, Status> {
    let p_url = Url::parse(url).map_err(|_| Status::UnprocessableEntity)?;

    let id_result = sqlx::query("SELECT id FROM urls_data WHERE url = $1;")
        .bind(&p_url.to_string())
        .fetch_one(&mut **db)
        .await;

    let id = match id_result {
        Ok(row) => match row.try_get::<String, _>(0) {
            Ok(id) => return Ok(format!("http://localhost:8001/{}", id)),
            Err(_) => nanoid::nanoid!(6),
        },
        Err(Error::RowNotFound) => nanoid::nanoid!(6),
        Err(e) => {
            println!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let _ = sqlx::query("INSERT INTO urls_data (id, url) VALUES ($1, $2);")
        .bind(&id)
        .bind(&p_url.to_string())
        .execute(&mut **db)
        .await
        .map_err(|e| {
            println!("Error inserting data: {}", e);
            Status::InternalServerError
        });

    Ok(format!("Returned ID: {}", id))
}

#[get("/")]
fn landing() -> String {
    "Hello, world!".to_string()
}

#[get("/<url_id>")]
fn shortener(url_id: &str) -> String {
    format!("Hello, world! -> {}", url_id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Urls::init())
        .mount("/", routes![shortener, landing, short_creation])
}

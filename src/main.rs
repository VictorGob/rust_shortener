#[macro_use]
extern crate rocket;

use postgres::{Client, NoTls};
use rocket::http::Status;
use url::Url;

#[get("/<url_id>")]
fn shortener(url_id: &str) -> String {
    format!("Hello, world! -> {}", url_id)
}

#[post("/", data = "<url>")]
fn short_creation(url: &str) -> Result<String, Status> {
    let id = &nanoid::nanoid!(6);
    let p_url = Url::parse(&url).map_err(|_| Status::UnprocessableEntity)?;
}

#[get("/")]
fn landing() -> String {
    "Hello, world!".to_string()
}

fn check_database() -> Result<(), String> {
    println!("Checking database connection...");
    let mut client = match Client::connect(
        "host=localhost user=postgres password=local_url_shortener",
        NoTls,
    ) {
        Ok(client) => client,
        Err(e) => {
            return Err(format!("Error connecting to database: {}", e));
        }
    };

    let query = "SELECT EXISTS (
        SELECT 1
        FROM   information_schema.tables 
        WHERE  table_name = 'urls_data'
     );";
    match client.query_one(query, &[]) {
        Ok(row) => {
            let exists: bool = row.get(0);
            if exists {
                println!("The table 'urls_data' exists.");
            } else {
                return Err("The table 'urls_data' does not exist.".to_string());
            }
        }
        Err(e) => {
            return Err(format!("Error executing query: {}", e));
        }
    }

    Ok(())
}

#[launch]
fn rocket() -> _ {
    let _ = check_database();
    rocket::build().mount("/", routes![shortener, landing, short_creation])
}

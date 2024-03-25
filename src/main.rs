#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;

#[get("/<url_id>")]
fn shortener(url_id: &str) -> String {
    format!("Hello, world! -> {}", url_id)
}

#[post("/", data = "<url>")]
fn short_creation(url: &str) -> String {
    format!("Hello, world! -> {}", url)
}

#[get("/")]
fn landing() -> String {
    "Hello, world!".to_string()
}

fn check_database() -> Result<(), String> {
    println!("Checking database connection...");
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment())
        .attach(AdHoc::try_on_ignite(
            "Database Connection Check",
            |rocket| async {
                match check_database() {
                    Ok(_) => Ok(rocket),
                    Err(_) => Err(rocket),
                }
            },
        ))
        .mount("/", routes![shortener, landing, short_creation])
}

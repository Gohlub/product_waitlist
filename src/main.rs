mod api_handler;
mod db_handler;
mod template;

#[macro_use]
extern crate rocket;

use api_handler::request_handler::{create_entry};
use api_handler::request_handler::{send_mail};
use crate::db_handler::db_manipulation::MongoInstance;


#[launch]
    fn rocket() -> _ {
        let db = MongoInstance::init().unwrap_or_else(|err| {
            eprintln!("Failed to initialize MongoDB: {}", err);
            std::process::exit(1);
        });
        rocket::build()
            .manage(db)
            .mount("/", routes![create_entry])
            .mount("/", routes![send_mail])
    }
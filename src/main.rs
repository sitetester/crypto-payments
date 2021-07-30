#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

pub mod models;
pub mod schema;

mod crypto_transaction;
use self::crypto_transaction::importer::TransactionsImporter;
use self::crypto_transaction::processor::TransactionsProcessor;

fn main() {
    let cn = establish_connection();

    TransactionsImporter::import(&cn);
    TransactionsProcessor::process(&cn);
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


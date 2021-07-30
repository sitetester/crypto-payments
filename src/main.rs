#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

use crate::transactions_importer::TransactionsImporter;
use crate::transactions_processor::TransactionsProcessor;

pub mod models;
pub mod schema;

mod transactions_importer;
mod transactions_processor;
mod transactions_repository;

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


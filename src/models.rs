use serde::{Deserialize, Serialize};

use crate::schema::*;
use diesel::sql_types::{Integer, Double};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct CryptoTransaction {

    #[serde(skip_deserializing)]
    pub id: i32,

    #[serde(rename(deserialize = "involvesWatchonly"))]
    pub involves_watchonly: bool,

    pub account: String,
    pub address: String,
    pub category: String,
    pub amount: f64,
    pub label: String,
    pub confirmations: i32,
    pub blockhash: String,
    pub blockindex: i32,
    pub blocktime: i64,
    pub txid: String,
    pub vout: i32,
    pub walletconflicts: Vec<String>,
    pub time: i64,
    pub timereceived: i64,

    #[serde(rename(deserialize = "bip125-replaceable"))]
    pub bip125_replaceable: String
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewCryptoTransaction<'a> {
    pub involves_watchonly: &'a i32,
    pub account: &'a str,
    pub address: &'a str,
    pub category: &'a str,
    pub amount: &'a f64,
    pub label: &'a str,
    pub confirmations: &'a i32,
    pub blockhash: &'a str,
    pub blockindex: &'a i32,
    pub blocktime: &'a i64,
    pub txid: &'a str,
    pub vout: &'a i32,
    pub walletconflicts: &'a str,
    pub time: &'a i64,
    pub timereceived: &'a i64,
    pub bip125_replaceable: &'a str,
}

#[derive(Queryable, Debug, QueryableByName)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str
}


#[derive(Queryable, Debug, QueryableByName)]
#[table_name = "transactions"]
pub struct Deposit {
    pub(crate) address: String,

    #[sql_type = "Integer"]
    pub(crate) count: i32,

    #[sql_type = "Double"]
    pub(crate) sum: f64,

}


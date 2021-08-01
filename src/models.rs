use diesel::sql_types::{Double, Integer};
use serde::{Deserialize, Serialize};

use crate::schema::*;

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
pub struct NewCryptoTransaction {
    pub involves_watchonly: i32,
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
    pub walletconflicts: String,
    pub time: i64,
    pub timereceived: i64,
    pub bip125_replaceable: String,
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


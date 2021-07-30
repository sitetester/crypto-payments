use std::fs::File;
use std::path::Path;

use diesel::{RunQueryDsl, SqliteConnection};
use serde::Deserialize;

use crate::models::{CryptoTransaction, NewCryptoTransaction};
use crate::schema::transactions;

pub struct TransactionsImporter {}

impl TransactionsImporter {
    // https://docs.rs/diesel/1.4.7/diesel/dsl/fn.insert_into.html
    // https://docs.rs/diesel/1.4.7/diesel/dsl/fn.insert_into.html#using-struct-for-values
    pub fn import(cn: &SqliteConnection) {
        for n in 1..=2 {
            let path = format!("./data/transactions-{}.json", n);

            let json_file_path = Path::new(&path);
            let file = File::open(json_file_path).expect("file not found");

            let transactions: CryptoTransactions = serde_json::from_reader(file).expect("error parsing JSON file");
            for transaction in transactions.transactions {
                let tx = NewCryptoTransaction {
                    involves_watchonly: &(transaction.involves_watchonly as i32),
                    account: &*transaction.account,
                    address: &*transaction.address,
                    category: &*transaction.category,
                    amount: &transaction.amount,
                    label: &transaction.label,
                    confirmations: &transaction.confirmations,
                    blockhash: &transaction.blockhash,
                    blockindex: &transaction.blockindex,
                    blocktime: &transaction.blocktime,
                    txid: &transaction.txid,
                    vout: &transaction.vout,
                    walletconflicts: &transaction.walletconflicts.join(","),
                    time: &transaction.time,
                    timereceived: &transaction.timereceived,
                    bip125_replaceable: &transaction.bip125_replaceable,
                };

                diesel::insert_into(transactions::table)
                    .values(&tx)
                    .execute(cn)
                    .expect("Error saving new transaction!");
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CryptoTransactions {
    transactions: Vec<CryptoTransaction>,
}
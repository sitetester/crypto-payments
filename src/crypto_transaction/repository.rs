use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use diesel::dsl::sql;
use diesel::expression_methods::ExpressionMethods;
use diesel::prelude::*;
use diesel::sql_types::{Double, Integer};

use crate::models::*;
use crate::schema::transactions::dsl::*;

pub struct TransactionsRepository {}

// https://docs.rs/diesel/0.16.0/diesel/expression/dsl/fn.sum.html
// https://docs.diesel.rs/diesel/expression_methods/trait.ExpressionMethods.html
// https://docs.diesel.rs/diesel/expression_methods/trait.ExpressionMethods.html#method.eq_any
impl TransactionsRepository {
    pub fn find_deposits_by_addresses(addresses: &Vec<String>, cn: &SqliteConnection) -> Vec<Deposit> {
        let query = transactions.select((address, sql::<Integer>("COUNT(confirmations) AS count"), sql::<Double>("SUM(amount) AS sum")))
            .group_by(address)
            .filter(address.eq_any(addresses))
            .filter(confirmations.ge(6))
            .filter(amount.gt(0.0))
            .filter(category.eq("receive"))
            .distinct()
            ;

        query.load::<Deposit>(cn).unwrap()
    }

    // https://docs.diesel.rs/diesel/expression_methods/trait.ExpressionMethods.html#method.ne_all
    pub fn find_deposits_without_known_addresses(addresses: &Vec<String>, cn: &SqliteConnection) -> Vec<Deposit> {
        let query = transactions.select((address, sql::<Integer>("COUNT(confirmations) AS count"), sql::<Double>("SUM(amount) AS sum")))
            .group_by(address)
            .filter(address.ne_all(addresses))
            .filter(confirmations.ge(6))
            .filter(amount.gt(0.0))
            .filter(category.eq("receive"))
            .distinct()
            ;

        query.load::<Deposit>(cn).unwrap()
    }

    pub fn smallest_or_largest_valid_deposit(cn: &SqliteConnection, exp: &str) -> Deposit {
        let query = transactions.select((address, confirmations, sql::<Double>(exp)))
            .filter(confirmations.ge(6))
            .filter(amount.gt(0.0))
            .filter(category.eq("receive"))
            .distinct()
            ;

        query.first::<Deposit>(cn).unwrap()
    }
}
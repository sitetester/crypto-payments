use diesel::SqliteConnection;

use crate::models::Deposit;

use super::repository::TransactionsRepository;

pub struct TransactionsProcessor {}

impl TransactionsProcessor {
    pub fn process(cn: &SqliteConnection) {
        let address_customer = [
            ("mvd6qFeVkqH6MNAS2Y2cLifbdaX5XUkbZJ".to_string(), "Wesley Crusher"),
            ("mmFFG4jqAtw9MoCC88hw5FNfreQWuEHADp".to_string(), "Leonard McCoy"),
            ("mzzg8fvHXydKs8j9D2a8t7KpSXpGgAnk4n".to_string(), "Jonathan Archer"),
            ("2N1SP7r92ZZJvYKG2oNtzPwYnzw62up7mTo".to_string(), "Jadzia Dax"),
            ("mutrAf4usv3HKNdpLwVD4ow2oLArL6Rez8".to_string(), "Montgomery Scott"),
            ("miTHhiX3iFhVnAEecLjybxvV5g8mKYTtnM".to_string(), "James T. Kirk"),
            ("mvcyJMiAcSXKAEsQxbW9TYZ369rsMG6rVV".to_string(), "Spock")
        ];

        let addresses: Vec<String> = address_customer.iter().map(|(address, _)| address.into()).collect();

        Self::print_known_deposits(
            &address_customer,
            TransactionsRepository::find_deposits_by_addresses(&addresses, &cn),
        );

        Self::print_unknown_deposits(
            TransactionsRepository::find_deposits_without_known_addresses(&addresses, &cn),
        );

        let smallest = TransactionsRepository::smallest_or_largest_valid_deposit(&cn, "MIN(amount) AS amount");
        println!("Smallest valid deposit: {}", smallest.sum);

        let largest = TransactionsRepository::smallest_or_largest_valid_deposit(&cn, "MAX(amount) AS amount");
        println!("Largest valid deposit: {}", largest.sum);
    }


    // returned data order could be different from given addresses sort order
    fn print_known_deposits(address_customer: &[(String, &str)], deposits: Vec<Deposit>) {
        for t in address_customer {
            for deposit in &deposits {
                if deposit.address.eq(&t.0) {
                    println!("Deposited for {}: count={} sum={}", t.1, deposit.count, deposit.sum);
                    break;
                }
            }
        }
    }

    fn print_unknown_deposits(known_deposits: Vec<Deposit>) {
        let mut count = 0;
        let mut sum = 0.0;

        for deposit in &known_deposits {
            count += deposit.count;
            sum += deposit.sum;
        }

        println!("Deposited without reference: count={} sum={}", count, sum);
    }
}



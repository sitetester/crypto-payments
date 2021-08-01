table! {
    transactions (id) {
        id -> Integer,
        involves_watchonly -> Integer,
        account -> Text,
        address -> Text,
        category -> Text,
        amount -> Double,
        label -> Text,
        confirmations -> Integer,
        blockhash -> Text,
        blockindex -> Integer,
        blocktime -> BigInt,
        txid -> Text,
        vout -> Integer,
        walletconflicts -> Text,
        time -> BigInt,
        timereceived -> BigInt,
        bip125_replaceable -> Text,
    }
}

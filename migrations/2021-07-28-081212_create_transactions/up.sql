CREATE TABLE transactions (
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    involves_watchonly INT NOT NULL,
    account           VARCHAR NOT NULL,
    address           VARCHAR NOT NULL,
    category          VARCHAR NOT NULL,
    amount            DOUBLE NOT NULL,
    label             VARCHAR NOT NULL,
    confirmations     INT NOT NULL,
    blockhash         VARCHAR NOT NULL,
    blockindex        INT NOT NULL,
    blocktime         BIGINT NOT NULL,
    txid              VARCHAR NOT NULL,
    vout              INT NOT NULL,
    walletconflicts   VARCHAR NOT NULL,
    `time`            BIGINT NOT NULL,
    timereceived      BIGINT NOT NULL,
    bip125_replaceable VARCHAR NOT NULL
);

CREATE TABLE posts (
  id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR NOT NULL,
  body VARCHAR NOT NULL

)

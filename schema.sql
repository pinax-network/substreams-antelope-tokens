-----------------------------------------------------------
-- Tables to store the raw events without any processing --
-----------------------------------------------------------

CREATE TABLE IF NOT EXISTS transfer_events
(
    id           String,
    -- trace information
    trx_id       String,
    action_index UInt32,
    -- contract & scope --
    contract     FixedString(12),
    action       String,
    symcode      String,
    -- data payload --
    from         FixedString(12),
    to           FixedString(12),
    quantity     String,
    memo         String,
    -- extras --
    precision    UInt32,
    amount       Int64,
    value        Float64,
    -- meta --
    block_num    UInt64,
    timestamp    DateTime64(3)
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

CREATE TABLE IF NOT EXISTS account_events
(
    id            String,
    -- trace information
    trx_id        String,
    action_index  UInt32,
    -- contract & scope --
    contract      FixedString(12),
    symcode       String,
    -- data payload --
    account       FixedString(12),
    balance       String,
    balance_delta Int64,
    -- extras --
    precision     UInt32,
    amount        Int64,
    value         Float64,
    -- meta --
    block_num     UInt64,
    timestamp     DateTime64(3)
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

CREATE TABLE IF NOT EXISTS token_events
(
    id           String,
    -- trace information --
    trx_id       String,
    action_index UInt32,
    -- contract & scope --
    contract     FixedString(12),
    symcode      String,
    -- data payload --
    issuer       FixedString(12),
    max_supply   String,
    supply       String,
    supply_delta Int64,
    -- extras --
    precision    UInt32,
    amount       Int64,
    value        Float64,
    -- meta --
    block_num    UInt64,
    timestamp    DateTime64(3)
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);


-----------------------------------------------
-- Tables to store the extracted information --
-----------------------------------------------

CREATE TABLE IF NOT EXISTS account_balances
(
    account              FixedString(12),

    contract             FixedString(12),
    symcode              String,
    balance              String,

    precision            UInt32,
    amount               Int64,
    value                Float64,

    updated_at_block_num UInt64,
    updated_at_timestamp DateTime64(3)
)
    ENGINE = ReplacingMergeTree(updated_at_block_num)
        PRIMARY KEY (account, contract, symcode)
        ORDER BY (account, contract, symcode);


---------------------------------------------------------
-- Materialized views to populate the extracted tables --
---------------------------------------------------------

CREATE MATERIALIZED VIEW account_balances_mv
    TO account_balances
AS
SELECT account,
       contract,
       symcode,
       balance,
       precision,
       amount,
       value,
       block_num,
       timestamp
FROM account_events;


-- TABLE VIEWS (transfers) --
-- From --
CREATE MATERIALIZED VIEW transfers_from_mv
            ENGINE = ReplacingMergeTree(timestamp)
                ORDER BY (from, contract)
AS
SELECT *
FROM transfer_events;

-- TODO: Useful ? --
-- OPTIMIZE TABLE transfers_from_mv FINAL --

-- To --
CREATE MATERIALIZED VIEW transfers_to_mv
            ENGINE = ReplacingMergeTree()
                ORDER BY (chain, to, contract)
AS
SELECT *
FROM transfers

-- Timestamp --
         CREATE MATERIALIZED VIEW transfers_timestamp_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, timestamp)
    AS
SELECT *
FROM transfers

-- BlockNumber --
         CREATE MATERIALIZED VIEW transfers_blocknumber_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, block_number)
    AS
SELECT *
FROM transfers

-- Contract --
         CREATE MATERIALIZED VIEW transfers_contract_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, contract)
    AS
SELECT *
FROM transfers

-- Symcode --
         CREATE MATERIALIZED VIEW transfers_symcode_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, symcode)
    AS
SELECT *
FROM transfers

         -- TABLE VIEWS (accounts) --
-- Balance --
         CREATE MATERIALIZED VIEW accounts_balance_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, account, balance)
    AS
SELECT *
FROM accounts

         -- TABLE VIEWS (stats) --
-- Supply --
         CREATE MATERIALIZED VIEW stats_supply_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, contract, supply, max_supply)
    AS
SELECT *
FROM stats
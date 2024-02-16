-- Table for transfers --
CREATE TABLE IF NOT EXISTS transfers (
    -- trace information
    trx_id String,
    action_index UInt32,
    -- contract & scope --
    contract FixedString(12),
    action String,
    symcode String,
    -- data payload --
    from FixedString(12),
    to FixedString(12),
    quantity String,
    memo String,
    -- extras --
    precision UInt32,
    amount Int64,
    value Float64,
)
ENGINE = ReplacingMergeTree()
-- primary key = trx_id + action_index --
PRIMARY KEY (id)
ORDER BY (id)

-- Table for accounts --
CREATE TABLE IF NOT EXISTS accounts (
    -- trace information --
    trx_id String,
    action_index UInt32,

    -- contract & scope --
    contract FixedString(12),
    symcode String,

    -- data payload --
    account FixedString(12),
    balance String,
    balance_delta Int64,

    -- extras --
    precision UInt32,
    amount Int64,
    value Float64,
)
ENGINE = ReplacingMergeTree()
-- primary key = trx_id + action_index --
PRIMARY KEY (id)
ORDER BY (id)

-- Table for stats --
CREATE TABLE IF NOT EXISTS stats (
    -- trace information --
    trx_id String,
    action_index UInt32,

    -- contract & scope --
    contract FixedString(12),
    symcode String,

    -- data payload --
    issuer FixedString(12),
    max_supply String,
    supply String,
    supply_delta Int64,

    -- extras --
    precision UInt32,
    amount Int64,
    value Float64,
)
ENGINE = ReplacingMergeTree()
-- primary key = trx_id + action_index --
PRIMARY KEY (id)
ORDER BY (id)

-- TABLE VIEWS (transfers) --
-- From --
CREATE MATERIALIZED VIEW transfers_from_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, from, contract)
AS SELECT
    *
FROM transfers

-- TODO: Useful ? --
-- OPTIMIZE TABLE transfers_from_mv FINAL --

-- To --
CREATE MATERIALIZED VIEW transfers_to_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, to, contract)
AS SELECT
    *
FROM transfers

-- Timestamp --
CREATE MATERIALIZED VIEW transfers_timestamp_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, timestamp)
AS SELECT
    *
FROM transfers

-- BlockNumber --
CREATE MATERIALIZED VIEW transfers_blocknumber_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, block_number)
AS SELECT
    *
FROM transfers

-- Contract --
CREATE MATERIALIZED VIEW transfers_contract_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, contract)
AS SELECT
    *
FROM transfers

-- Symbol --
CREATE MATERIALIZED VIEW transfers_symbol_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, symbol)
AS SELECT
    *
FROM transfers

-- TABLE VIEWS (accounts) --
-- Balance --
CREATE MATERIALIZED VIEW accounts_balance_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, account, balance)
AS SELECT
    *
FROM accounts

-- TABLE VIEWS (stats) --
-- Supply --
CREATE MATERIALIZED VIEW stats_supply_mv
ENGINE = ReplacingMergeTree()
ORDER BY (chain, contract, supply, max_supply)
AS SELECT
    *
FROM stats
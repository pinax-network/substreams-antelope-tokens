CREATE TABLE IF NOT EXISTS transfers (
    trx_id String,
    action_index UInt32,
    contract FixedString(12),
    action String,
    symcode String,
    from FixedString(12),
    to FixedString(12),
    memo String,
    quantity String,
    amount Int64,
    precision UInt32,
    value Float64,
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain); # Created by Clickhouse Sink

CREATE TABLE IF NOT EXISTS accounts (
    trx_id String,
    action_index UInt32,
    contract FixedString(12),
    symcode String,
    amount Int64,
    precision UInt32,
    value Float64,
    account FixedString(12),
    balance String,
    balance_delta Int64,
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain); # Created by Clickhouse Sink

CREATE TABLE IF NOT EXISTS stats (
    trx_id String,
    action_index UInt32,
    contract FixedString(12),
    symcode String,
    issuer FixedString(12),
    max_supply String,
    supply String,
    supply_delta Int64,
    precision UInt32,
    amount Int64,
    value Float64,
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain) # Created by Clickhouse Sink
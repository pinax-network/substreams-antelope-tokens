CREATE TABLE IF NOT EXISTS transfers (
    trx_id String,
    action_ordinal String,
    contract String,
    action String,
    symcode String,
    from String,
    to String,
    memo String,
    quantity String,
    amount String,
    precision String,
    value String,
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain); # Created by Clickhouse Sink

CREATE TABLE IF NOT EXISTS accounts (
    trx_id String,
    action_index String,
    contract String,
    symcode String,
    amount String,
    precision String,
    value String,
    account String,
    balance String,
    balance_delta String,
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain); # Created by Clickhouse Sink

CREATE TABLE IF NOT EXISTS stats (
    trx_id String,
    action_index String,
    contract String,
    symcode String,
    issuer String,
    max_supply String,
    supply String,
    supply_delta String,
    precision String,
    amount String,
    value String,
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain) # Created by Clickhouse Sink
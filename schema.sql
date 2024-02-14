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
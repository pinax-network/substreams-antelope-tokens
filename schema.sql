-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------

CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

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
    timestamp    DateTime
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
    timestamp     DateTime
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
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);


-----------------------------------------------
-- Tables to store the extracted information --
-----------------------------------------------

-- Table to store up to date balances per account and token --
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
    updated_at_timestamp DateTime
)
    ENGINE = ReplacingMergeTree(updated_at_block_num)
        PRIMARY KEY (account, contract, symcode)
        ORDER BY (account, contract, symcode);

-- Table to store up to date token supplies --
CREATE TABLE IF NOT EXISTS token_balances
(
    contract             FixedString(12),
    symcode              String,

    issuer               FixedString(12),
    max_supply           String,
    supply               String,

    precision            UInt32,
    amount               Int64,
    value                Float64,

    updated_at_block_num UInt64,
    updated_at_timestamp DateTime
)
    ENGINE = ReplacingMergeTree(updated_at_block_num)
        PRIMARY KEY (contract, symcode, issuer)
        ORDER BY (contract, symcode, issuer);

-- Table to store token transfers primarily indexed by the 'from' field --
CREATE TABLE IF NOT EXISTS transfers_from
(
    trx_id       String,
    action_index UInt32,

    contract     FixedString(12),
    action       String,
    symcode      String,

    from         FixedString(12),
    to           FixedString(12),
    quantity     String,
    memo         String,

    precision    UInt32,
    amount       Int64,
    value        Float64,

    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree(block_num)
        PRIMARY KEY (from, to, trx_id, action_index)
        ORDER BY (from, to, trx_id, action_index);

-- Table to store token transfers primarily indexed by the 'to' field --
CREATE TABLE IF NOT EXISTS transfers_to
(
    trx_id       String,
    action_index UInt32,

    contract     FixedString(12),
    action       String,
    symcode      String,

    from         FixedString(12),
    to           FixedString(12),
    quantity     String,
    memo         String,

    precision    UInt32,
    amount       Int64,
    value        Float64,

    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree(block_num)
        PRIMARY KEY (to, from, trx_id, action_index)
        ORDER BY (to, from, trx_id, action_index);

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
       block_num AS updated_at_block_num,
       timestamp AS updated_at_timestamp
FROM account_events;

CREATE MATERIALIZED VIEW token_balances_mv
    TO token_balances
AS
SELECT contract,
       symcode,
       issuer,
       max_supply,
       supply,
       precision,
       amount,
       value,
       block_num AS updated_at_block_num,
       timestamp AS updated_at_timestamp
FROM token_events;

CREATE MATERIALIZED VIEW transfers_from_mv
    TO transfers_from
AS
SELECT trx_id,
       action_index,
       contract,
       action,
       symcode,
       from,
       to,
       quantity,
       memo,
       precision,
       amount,
       value,
       block_num,
       timestamp
FROM transfer_events;

CREATE MATERIALIZED VIEW transfers_to_mv
    TO transfers_to
AS
SELECT trx_id,
       action_index,
       contract,
       action,
       symcode,
       from,
       to,
       quantity,
       memo,
       precision,
       amount,
       value,
       block_num,
       timestamp
FROM transfer_events;
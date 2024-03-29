use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_entity_change::pb::entity::EntityChanges;

use crate::eosio_token::{Accounts, Stats, TransferEvents};
use crate::utils::to_key;

#[substreams::handlers::map]
pub fn graph_out(
    map_accounts: Accounts,
    map_stats: Stats,
    map_transfers: TransferEvents,
) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    for account in map_accounts.items {
        let key = to_key(&account.trx_id, account.action_index);
        tables
            .create_row("accounts", key)
            // transaction
            .set("trx_id", account.trx_id.to_string())
            .set("action_index", account.action_index.to_string())
            // contract & scope
            .set("contract", account.contract.to_string())
            .set("symcode", account.symcode.to_string())
            // data payload
            .set("account", account.account.to_string())
            .set("balance", account.balance.to_string())
            .set("balance_delta", account.balance_delta.to_string())
            // extras
            .set("precision", account.precision.to_string())
            .set("amount", account.amount.to_string())
            .set("value", account.value.to_string());
    }

    for stat in map_stats.items {
        let key = to_key(&stat.trx_id, stat.action_index);
        tables
            .create_row("stats", key)
            // transaction
            .set("trx_id", stat.trx_id.to_string())
            .set("action_index", stat.action_index.to_string())
            // contract & scope
            .set("contract", stat.contract.to_string())
            .set("symcode", stat.symcode.to_string())
            // data payload
            .set("issuer", stat.issuer.to_string())
            .set("max_supply", stat.max_supply.to_string())
            .set("supply", stat.supply.to_string())
            .set("supply_delta", stat.supply_delta.to_string())
            // extras
            .set("precision", stat.precision.to_string())
            .set("amount", stat.amount.to_string())
            .set("value", stat.value.to_string());
    }

    for transfer in map_transfers.items {
        let key = to_key(&transfer.trx_id, transfer.action_index);
        tables
            .create_row("transfers", key)
            // transaction
            .set("trx_id", transfer.trx_id.to_string())
            .set("action_index", transfer.action_index.to_string())
            // contract & scope
            .set("contract", transfer.contract.to_string())
            .set("action", transfer.action.to_string())
            .set("symcode", transfer.symcode.to_string())
            // data payload
            .set("from", transfer.from.to_string())
            .set("to", transfer.to.to_string())
            .set("memo", transfer.memo.to_string())
            .set("quantity", transfer.quantity.to_string())
            // extras
            .set("amount", transfer.amount.to_string())
            .set("precision", transfer.precision.to_string())
            .set("value", transfer.value.to_string());
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn ch_out(
    map_accounts: Accounts,
    map_stats: Stats,
    map_transfers: TransferEvents,
) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();

    for account in map_accounts.items {
        let key = to_key(&account.trx_id, account.action_index);
        tables
            .push_change("account_events", key, 0, table_change::Operation::Create)
            // transaction
            .change("trx_id", ("", account.trx_id.to_string().as_str()))
            .change(
                "action_index",
                ("", account.action_index.to_string().as_str()),
            )
            // contract & scope
            .change("contract", ("", account.contract.to_string().as_str()))
            .change("symcode", ("", account.symcode.to_string().as_str()))
            // data payload
            .change("account", ("", account.account.to_string().as_str()))
            .change("balance", ("", account.balance.to_string().as_str()))
            .change(
                "balance_delta",
                ("", account.balance_delta.to_string().as_str()),
            )
            // extras
            .change("precision", ("", account.precision.to_string().as_str()))
            .change("amount", ("", account.amount.to_string().as_str()))
            .change("value", ("", account.value.to_string().as_str()))
            .change("block_num", ("", account.block_num.to_string().as_str()))
            .change(
                "timestamp",
                ("", account.timestamp.unwrap().to_string().as_str()),
            );
    }

    for stat in map_stats.items {
        let key = to_key(&stat.trx_id, stat.action_index);
        tables
            .push_change("token_events", key, 0, table_change::Operation::Create)
            // transaction
            .change("trx_id", ("", stat.trx_id.to_string().as_str()))
            .change("action_index", ("", stat.action_index.to_string().as_str()))
            // contract & scope
            .change("contract", ("", stat.contract.to_string().as_str()))
            .change("symcode", ("", stat.symcode.to_string().as_str()))
            // data payload
            .change("issuer", ("", stat.issuer.to_string().as_str()))
            .change("max_supply", ("", stat.max_supply.to_string().as_str()))
            .change("supply", ("", stat.supply.to_string().as_str()))
            .change("supply_delta", ("", stat.supply_delta.to_string().as_str()))
            // extras
            .change("precision", ("", stat.precision.to_string().as_str()))
            .change("amount", ("", stat.amount.to_string().as_str()))
            .change("value", ("", stat.value.to_string().as_str()))
            .change("block_num", ("", stat.block_num.to_string().as_str()))
            .change(
                "timestamp",
                ("", stat.timestamp.unwrap().to_string().as_str()),
            );
    }

    for transfer in map_transfers.items {
        let key = to_key(&transfer.trx_id, transfer.action_index);
        tables
            .push_change("transfer_events", key, 0, table_change::Operation::Create)
            // transaction
            .change("trx_id", ("", transfer.trx_id.to_string().as_str()))
            .change(
                "action_index",
                ("", transfer.action_index.to_string().as_str()),
            )
            // contract & scope
            .change("contract", ("", transfer.contract.to_string().as_str()))
            .change("action", ("", transfer.action.to_string().as_str()))
            .change("symcode", ("", transfer.symcode.to_string().as_str()))
            // data payload
            .change("from", ("", transfer.from.to_string().as_str()))
            .change("to", ("", transfer.to.to_string().as_str()))
            .change("memo", ("", transfer.memo.to_string().as_str()))
            .change("quantity", ("", transfer.quantity.to_string().as_str()))
            // extras
            .change("amount", ("", transfer.amount.to_string().as_str()))
            .change("precision", ("", transfer.precision.to_string().as_str()))
            .change("value", ("", transfer.value.to_string().as_str()))
            .change("block_num", ("", transfer.block_num.to_string().as_str()))
            .change(
                "timestamp",
                ("", transfer.timestamp.unwrap().to_string().as_str()),
            );
    }

    Ok(tables)
}

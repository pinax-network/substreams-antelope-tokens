use std::collections::HashMap;
use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_entity_change::pb::entity::EntityChanges;

use crate::eosio_token::Events;
use crate::utils::to_key;

#[substreams::handlers::map]
pub fn graph_out(
    map_events: Events,
) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    for account in map_events.balance_changes {
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

    for stat in map_events.supply_changes {
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

    for transfer in map_events.transfers {
        let key = to_key(&transfer.trx_id, transfer.action_index);
        tables
            .create_row("transfers", key)
            // transaction
            .set("trx_id", transfer.trx_id.to_string())
            .set("action_index", transfer.action_index.to_string())
            // contract & scope
            .set("contract", transfer.contract.to_string())
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

    for issue in map_events.issues {
        let key = to_key(&issue.trx_id, issue.action_index);
        tables
            .create_row("issues", key)
            // transaction
            .set("trx_id", issue.trx_id.to_string())
            .set("action_index", issue.action_index.to_string())
            // contract & scope
            .set("contract", issue.contract.to_string())
            .set("symcode", issue.symcode.to_string())
            // data payload
            .set("issuer", issue.issuer.to_string())
            .set("to", issue.to.to_string())
            .set("memo", issue.memo.to_string())
            .set("quantity", issue.quantity.to_string())
            // extras
            .set("amount", issue.amount.to_string())
            .set("precision", issue.precision.to_string())
            .set("value", issue.value.to_string());
    }

    for retire in map_events.retires {
        let key = to_key(&retire.trx_id, retire.action_index);
        tables
            .create_row("retires", key)
            // transaction
            .set("trx_id", retire.trx_id.to_string())
            .set("action_index", retire.action_index.to_string())
            // contract & scope
            .set("contract", retire.contract.to_string())
            .set("symcode", retire.symcode.to_string())
            // data payload
            .set("quantity", retire.quantity.to_string())
            .set("from", retire.from.to_string())
            .set("memo", retire.memo.to_string())
            // extras
            .set("amount", retire.amount.to_string())
            .set("precision", retire.precision.to_string())
            .set("value", retire.value.to_string());
    }

    for create in map_events.creates {
        let key = to_key(&create.trx_id, create.action_index);
        tables
            .create_row("creates", key)
            // transaction
            .set("trx_id", create.trx_id.to_string())
            .set("action_index", create.action_index.to_string())
            // contract & scope
            .set("contract", create.contract.to_string())
            .set("symcode", create.symcode.to_string())
            // data payload
            .set("issuer", create.issuer.to_string())
            .set("maximum_supply", create.maximum_supply.to_string())
            // extras
            .set("amount", create.amount.to_string())
            .set("precision", create.precision.to_string())
            .set("value", create.value.to_string());
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn ch_out(
    map_events: Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();

    for account in map_events.balance_changes {
        let keys = HashMap::from([
            ("account".to_string(), account.account.to_string()),
            ("block_num".to_string(), account.block_num.to_string()),
            ("trx_id".to_string(), account.trx_id),
            ("action_index".to_string(), account.action_index.to_string()),
        ]);

        tables
            .push_change_composite("account_events", keys, 0, table_change::Operation::Create)
            .change("contract", ("", account.contract.to_string().as_str()))
            .change("symcode", ("", account.symcode.to_string().as_str()))
            .change("balance", ("", account.balance.to_string().as_str()))
            .change("balance_delta", ("", account.balance_delta.to_string().as_str()))
            .change("precision", ("", account.precision.to_string().as_str()))
            .change("amount", ("", account.amount.to_string().as_str()))
            .change("value", ("", account.value.to_string().as_str()))
            .change("timestamp", ("", account.timestamp.unwrap().to_string().as_str()));
    }

    for stat in map_events.supply_changes {
        let keys = HashMap::from([
            ("contract".to_string(), stat.contract.to_string()),
            ("block_num".to_string(), stat.block_num.to_string()),
            ("trx_id".to_string(), stat.trx_id),
            ("action_index".to_string(), stat.action_index.to_string()),
        ]);

        tables
            .push_change_composite("token_supply_events", keys, 0, table_change::Operation::Create)
            .change("symcode", ("", stat.symcode.to_string().as_str()))
            .change("issuer", ("", stat.issuer.to_string().as_str()))
            .change("max_supply", ("", stat.max_supply.to_string().as_str()))
            .change("supply", ("", stat.supply.to_string().as_str()))
            .change("supply_delta", ("", stat.supply_delta.to_string().as_str()))
            .change("precision", ("", stat.precision.to_string().as_str()))
            .change("amount", ("", stat.amount.to_string().as_str()))
            .change("value", ("", stat.value.to_string().as_str()))
            .change("timestamp", ("", stat.timestamp.unwrap().to_string().as_str()));
    }

    for transfer in map_events.transfers {
        let keys = HashMap::from([
            ("trx_id".to_string(), transfer.trx_id),
            ("action_index".to_string(), transfer.action_index.to_string()),
        ]);

        tables
            .push_change_composite("transfer_events", keys, 0, table_change::Operation::Create)
            .change("contract", ("", transfer.contract.to_string().as_str()))
            .change("symcode", ("", transfer.symcode.to_string().as_str()))
            .change("from", ("", transfer.from.to_string().as_str()))
            .change("to", ("", transfer.to.to_string().as_str()))
            .change("memo", ("", transfer.memo.to_string().as_str()))
            .change("quantity", ("", transfer.quantity.to_string().as_str()))
            .change("amount", ("", transfer.amount.to_string().as_str()))
            .change("precision", ("", transfer.precision.to_string().as_str()))
            .change("value", ("", transfer.value.to_string().as_str()))
            .change("block_num", ("", transfer.block_num.to_string().as_str()))
            .change("timestamp", ("", transfer.timestamp.unwrap().to_string().as_str()));
    }

    Ok(tables)
}

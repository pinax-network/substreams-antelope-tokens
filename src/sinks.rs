use antelope::Asset;
use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

use crate::eosio_token::{Accounts, Stats, TransferEvents};

#[substreams::handlers::map]
pub fn graph_out(map_accounts: Accounts, map_transfers: TransferEvents) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();
    
    for account in map_accounts.items {
        tables
            .create_row("accounts", account.trx_id.as_str().to_string())
            
            // transaction
            .set("action_index", account.action_index.to_string())
            
            // contract & scope
            .set("contract", account.contract.to_string())
            
            // data payload
            .set("account", account.account.to_string())
            .set("balance", account.balance.to_string())
            .set("balance_delta", account.balance_delta.to_string())
            
            // extras
            .set("precision", account.precision.to_string())
            .set("amount", account.amount.to_string())
            .set("value", account.value.to_string());
    }

    for transfer in map_transfers.items {
        tables
            .create_row("transfers", transfer.trx_id.as_str())

            // transaction
            .set("trx_id", transfer.trx_id.to_string())
            .set("action_ordinal", transfer.action_ordinal.to_string())

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
pub fn kv_out(map_accounts: Accounts, map_stat: Stats) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    let mut ordinal = 1;
    for account in map_accounts.items {
        let asset = Asset::from(account.balance.as_str());
        let key = format!(
            "accounts:{}:{}:{}",
            account.account, account.contract, asset.symbol
        );
        kv_ops.push_new(key, asset.amount.to_string(), ordinal);
        ordinal += 1;
    }

    ordinal = 1;
    for stat in map_stat.items {
        let asset = Asset::from(stat.supply.as_str());
        let key = format!("stat:{}:{}", stat.contract, asset.symbol);
        kv_ops.push_new(key, asset.amount.to_string(), ordinal);
        ordinal += 1;
    }

    Ok(kv_ops)
}

#[substreams::handlers::map]
pub fn db_out(map_transfers: TransferEvents) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for transfer in map_transfers.items {
        let pk = format!("{}-{}", transfer.trx_id, transfer.action_ordinal);
        db_out
            .push_change("transfer", pk.as_str(), 0, table_change::Operation::Create)
            // transaction
            .change("trx_id", ("", transfer.trx_id.as_str()))
            .change("action_ordinal", ("", transfer.action_ordinal.to_string().as_str()))

            // contract & scope
            .change("contract", ("", transfer.contract.as_str()))
            .change("action", ("", transfer.action.as_str()))
            .change("symcode", ("", transfer.symcode.as_str()))

            // data payload
            .change("from", ("", transfer.from.as_str()))
            .change("to", ("", transfer.to.as_str()))
            .change("memo", ("", transfer.memo.as_str()))
            .change("quantity", ("", transfer.quantity.as_str()))

            // extras
            .change("amount", ("", transfer.amount.to_string().as_str()))
            .change("precision", ("", transfer.precision.to_string().as_str()))
            .change("value", ("", transfer.value.to_string().as_str()));
    }

    Ok(db_out)
}
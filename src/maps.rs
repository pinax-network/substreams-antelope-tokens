use antelope::Symbol;
use substreams::errors::Error;
// use substreams::log;
use substreams_antelope::pb::Block;

use crate::abi;
use crate::eosio_token::*;
use crate::utils;
use antelope::{Asset, Name, SymbolCode};

#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<Accounts, Error> {
    let mut items = vec![];

    for trx in block.executed_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key);
            let account = db_op.scope.clone();

            let old_data = abi::Account::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::Account::try_from(db_op.new_data_json.as_str()).ok();
            if old_data.is_none() && new_data.is_none() { continue; } // no data

            let old_balance = match &old_data {
                Some(data) => Some(Asset::from(data.balance.as_str())),
                None => None,
            };
            let new_balance = match &new_data {
                Some(data) => Some(Asset::from(data.balance.as_str())),
                None => None,
            };
            let precision = match new_balance.is_some() {
                true => new_balance.unwrap().symbol.precision(),
                false => old_balance.unwrap().symbol.precision(),
            };
            let sym = Symbol::from_precision(symcode, precision);
            let balance = match new_balance.is_some() {
                true => new_balance.unwrap(),
                false => Asset::from_amount(0, sym),
            };

            let balance_delta = match old_balance.is_some() {
                true => balance.amount - old_balance.unwrap().amount,
                false => balance.amount,
            };

            items.push(Account {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract,
                symcode: symcode.to_string(),

                // payload
                account,
                balance: balance.to_string(),
                balance_delta,

                // extras
                precision: precision.into(),
                amount: balance.amount,
                value: utils::to_value(balance),

                block_num: block.number as u64,
                timestamp: block.header.clone().unwrap().timestamp,
            });
        }
    }
    Ok(Accounts { items })
}

#[substreams::handlers::map]
fn map_stat(block: Block) -> Result<Stats, Error> {
    let mut items = vec![];

    for trx in block.executed_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "stat" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key);

            let old_data = abi::CurrencyStats::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::CurrencyStats::try_from(db_op.new_data_json.as_str()).ok();
            if old_data.is_none() && new_data.is_none() { continue; } // no data

            let old_supply = match &old_data {
                Some(data) => Some(Asset::from(data.supply.as_str())),
                None => None,
            };
            let new_supply = match &new_data {
                Some(data) => Some(Asset::from(data.supply.as_str())),
                None => None,
            };
            let precision = match new_supply.is_some() {
                true => new_supply.unwrap().symbol.precision(),
                false => old_supply.unwrap().symbol.precision(),
            };
            let sym = Symbol::from_precision(symcode, precision);
            let supply = match new_supply.is_some() {
                true => new_supply.unwrap(),
                false => Asset::from_amount(0, sym),
            };

            let supply_delta = match old_supply.is_some() {
                true => supply.amount - old_supply.unwrap().amount,
                false => supply.amount,
            };

            // Skip if no new data
            if new_data.is_none() { continue; }
            let data = new_data.unwrap();

            items.push(Stat {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract,
                symcode: symcode.to_string(),

                // payload
                issuer: data.issuer,
                max_supply: data.max_supply,
                supply: supply.to_string(),
                supply_delta,

                // extras
                precision: precision.into(),
                amount: supply.amount,
                value: utils::to_value(supply),

                block_num: block.number as u64,
                timestamp: block.header.clone().unwrap().timestamp,
            });
        }
    }
    Ok(Stats { items })
}

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let mut response = vec![];

    for trx in block.executed_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver { continue; }
            if action_trace.name != "transfer" { continue; }

            match abi::Transfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let quantity = Asset::from(data.quantity.as_str());
                    let symcode = quantity.symbol.code().to_string();
                    let precision = quantity.symbol.precision().into();
                    let amount = quantity.amount;
                    let contract = action_trace.account.clone();

                    // log::debug!("symcode: {:?}", symcode);

                    response.push(TransferEvent {
                        // trace information
                        trx_id: trx.id.clone(),
                        action_index: trace.action_ordinal,

                        // contract & scope
                        contract,
                        action: action_trace.name.clone(),
                        symcode,

                        // payload
                        from: data.from,
                        to: data.to,
                        quantity: data.quantity,
                        memo: data.memo,

                        // extras
                        precision,
                        amount,
                        value: utils::to_value(quantity),

                        block_num: block.number as u64,
                        timestamp: block.header.clone().unwrap().timestamp,
                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(TransferEvents { items: response })
}

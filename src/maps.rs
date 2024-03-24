use substreams::errors::Error;
use substreams::log;
use substreams_antelope::{pb::Block, decoder::decode};
use antelope::{Symbol, Asset, Name, SymbolCode};

use crate::abi;
use crate::eosio_token::*;
use crate::utils;

#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<Accounts, Error> {

    let items = block.transaction_traces().flat_map(|trx| {
        trx.db_ops.iter().filter_map(|db_op| {
            if db_op.table_name != "accounts" {
                return None;
            }

            let old_data = decode::<abi::types::Account>(&db_op.old_data_json).ok();
            let new_data = decode::<abi::types::Account>(&db_op.new_data_json).ok();

            let old_balance = old_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
                Ok(asset) => Some(asset),
                Err(e) => {
                    log::info!("Error parsing old balance asset in trx {}: {:?}", trx.id, e);
                    None
                }
            });
            let new_balance = new_data.as_ref().and_then(|data| match data.balance.parse::<Asset>() {
                Ok(asset) => Some(asset),
                Err(e) => {
                    log::info!("Error parsing new balance asset in trx {}: {:?}", trx.id, e);
                    None
                }
            });

            if old_balance.is_none() && new_balance.is_none() {
                return None;
            }

            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key);
            let precision = new_balance.unwrap_or_else(|| old_balance.unwrap()).symbol.precision();
            let sym = Symbol::from_precision(symcode, precision);
            let balance = new_balance.unwrap_or_else(|| Asset::from_amount(0, sym));
            let balance_delta = balance.amount - old_balance.unwrap_or_else(|| Asset::from_amount(0, sym)).amount;

            Some(Account {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract: db_op.code.clone(),
                symcode: symcode.to_string(),

                // payload
                account: db_op.scope.clone(),
                balance: balance.to_string(),
                balance_delta,

                // extras
                precision: precision.into(),
                amount: balance.amount,
                value: utils::to_value(&balance),

                block_num: block.number as u64,
                timestamp: block.header.as_ref().unwrap().timestamp.clone(),
            })
        })
    }).collect();

    Ok(Accounts { items })
}

#[substreams::handlers::map]
fn map_stat(block: Block) -> Result<Stats, Error> {
    let items = block.transaction_traces().flat_map(|trx| {
        trx.db_ops.iter().filter_map(|db_op| {
            if db_op.table_name != "stat" {
                return None;
            }

            let raw_primary_key = match db_op.primary_key.parse::<Name>() {
                Ok(name) => name.value,
                Err(e) => {
                    log::info!("Error parsing primary key as name in trx {}: {:?}", trx.id, e);
                    return None;
                }
            };

            let old_data = decode::<abi::types::CurrencyStats>(&db_op.old_data_json).ok();
            let new_data = decode::<abi::types::CurrencyStats>(&db_op.new_data_json).ok();

            let old_supply = old_data.as_ref().and_then(|data| match data.supply.parse::<Asset>() {
                Ok(asset) => Some(asset),
                Err(e) => {
                    log::info!("Error parsing old supply asset in trx {}: {:?}", trx.id, e);
                    None
                }
            });

            let new_supply = new_data.as_ref().and_then(|data| match data.supply.parse::<Asset>() {
                Ok(asset) => Some(asset),
                Err(e) => {
                    log::info!("Error parsing new supply asset in trx {}: {:?}", trx.id, e);
                    None
                }
            });

            if old_supply.is_none() && new_supply.is_none() {
                return None;
            }

            let symcode = SymbolCode::from(raw_primary_key);
            let precision = new_supply.unwrap_or_else(|| old_supply.unwrap()).symbol.precision();
            let sym = Symbol::from_precision(symcode, precision);
            let supply = new_supply.unwrap_or_else(|| Asset::from_amount(0, sym));
            let supply_delta = supply.amount - old_supply.unwrap_or_else(|| Asset::from_amount(0, sym)).amount;

            let data = new_data.unwrap_or_else(|| old_data.unwrap());

            Some(Stat {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract: db_op.code.clone(),
                symcode: symcode.to_string(),

                // payload
                issuer: data.issuer,
                max_supply: data.max_supply,
                supply: supply.to_string(),
                supply_delta,

                // extras
                precision: precision.into(),
                amount: supply.amount,
                value: utils::to_value(&supply),

                block_num: block.number as u64,
                timestamp: block.header.as_ref().unwrap().timestamp.clone(),
            })
        })
    })
    .collect();

    Ok(Stats { items })
}

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<TransferEvents, Error> {

    let items = block.transaction_traces().flat_map(|trx| {
        trx.action_traces.iter().filter_map(|trace| {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.name != "transfer" || action_trace.account != trace.receiver {
                return None;
            }

            match abi::actions::Transfer::decode(&trace) {
                Ok(data) => {
                    let quantity = match data.quantity.parse::<Asset>() {
            Ok(asset) => asset,
            Err(e) => {
                log::info!("Error parsing transfer asset in trx {}: {:?}", trx.id, e);
                return None;
            }
        };
        let symcode = quantity.symbol.code().to_string();
        let precision = quantity.symbol.precision().into();
        let amount = quantity.amount;

        Some(TransferEvent {
                        // trace information
            trx_id: trx.id.clone(),
                        action_index: trace.action_ordinal,

                        // contract & scope
                        contract: action_trace.account.clone(),
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
            value: utils::to_value(&quantity),

            block_num: block.number as u64,
            timestamp: block.header.as_ref().unwrap().timestamp.clone(),
                    })
                }
                Err(_) => return None,
            }
        })
    })
    .collect();

    Ok(TransferEvents { items })
}

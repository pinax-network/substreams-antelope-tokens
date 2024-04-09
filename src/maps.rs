use substreams::errors::Error;
use substreams::log;
use substreams_antelope::{pb::Block, decoder::decode};
use antelope::{Symbol, Asset, Name, SymbolCode};

use crate::abi;
use crate::eosio_token::*;
use crate::utils;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {

    let transfers = block.actions::<abi::actions::Transfer>(&[]).filter_map(|(action, action_trace, trx)| {

        let quantity = match action.quantity.parse::<Asset>() {
            Ok(asset) => asset,
            Err(e) => {
                log::info!("Error parsing transfer asset in trx {}: {:?}", trx.id, e);
                return None;
            }
        };

        Some(Transfer {
            trx_id: trx.id.clone(),
            action_index: action_trace.action_ordinal,

            contract: action_trace.action.as_ref().unwrap().account.clone(),
            symcode: quantity.symbol.code().to_string(),

            from: action.from,
            to: action.to,
            quantity: action.quantity,
            memo: action.memo,

            precision: quantity.symbol.precision().into(),
            amount: quantity.amount,
            value: utils::to_value(&quantity),

            block_num: block.number as u64,
            timestamp: block.header.as_ref().unwrap().timestamp.clone(),
        })
    })
    .collect();

    let issues = block.actions::<abi::actions::Issue>(&[]).filter_map(|(action, action_trace, trx)| {

        let quantity = match action.quantity.parse::<Asset>() {
            Ok(asset) => asset,
            Err(e) => {
                log::info!("Error parsing issue asset in trx {}: {:?}", trx.id, e);
                return None;
            }
        };

        Some(Issue {
            trx_id: trx.id.clone(),
            action_index: action_trace.action_ordinal,

            contract: action_trace.action.as_ref().unwrap().account.clone(),
            symcode: quantity.symbol.code().to_string(),

            issuer: action_trace.receiver.clone(),
            to: action.to,
            quantity: action.quantity,
            memo: action.memo,

            precision: quantity.symbol.precision().into(),
            amount: quantity.amount,
            value: utils::to_value(&quantity),

            block_num: block.number as u64,
            timestamp: block.header.as_ref().unwrap().timestamp.clone(),
        })
    })
    .collect();

    let retires = block.actions::<abi::actions::Retire>(&[]).filter_map(|(action, action_trace, trx)| {

        let quantity = match action.quantity.parse::<Asset>() {
            Ok(asset) => asset,
            Err(e) => {
                log::info!("Error parsing retire asset in trx {}: {:?}", trx.id, e);
                return None;
            }
        };

        Some(Retire {
            trx_id: trx.id.clone(),
            action_index: action_trace.action_ordinal,

            contract: action_trace.action.as_ref().unwrap().account.clone(),
            symcode: quantity.symbol.code().to_string(),

            from: action_trace.receiver.clone(),
            quantity: action.quantity,
            memo: action.memo,

            precision: quantity.symbol.precision().into(),
            amount: quantity.amount,
            value: utils::to_value(&quantity),

            block_num: block.number as u64,
            timestamp: block.header.as_ref().unwrap().timestamp.clone(),
        })
    })
    .collect();

    let creates = block.actions::<abi::actions::Create>(&[]).filter_map(|(action, action_trace, trx)| {

        let maximum_supply = match action.maximum_supply.parse::<Asset>() {
            Ok(asset) => asset,
            Err(e) => {
                log::info!("Error parsing create max supply asset in trx {}: {:?}", trx.id, e);
                return None;
            }
        };

        Some(Create {
            trx_id: trx.id.clone(),
            action_index: action_trace.action_ordinal,

            contract: action_trace.action.as_ref().unwrap().account.clone(),
            symcode: maximum_supply.symbol.code().to_string(),

            issuer: action_trace.receiver.clone(),
            maximum_supply: action.maximum_supply,

            precision: maximum_supply.symbol.precision().into(),
            amount: maximum_supply.amount,
            value: utils::to_value(&maximum_supply),

            block_num: block.number as u64,
            timestamp: block.header.as_ref().unwrap().timestamp.clone(),
        })
    })
    .collect();

    let balance_changes = block.transaction_traces().flat_map(|trx| {
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

            Some(BalanceChange {
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

    let supply_changes = block.transaction_traces().flat_map(|trx| {
        trx.db_ops.iter().filter_map(|db_op| {
            if db_op.table_name != "stat" {
                return None;
            }

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

            let symcode = SymbolCode::from(Name::from(db_op.primary_key.as_str()).value);
            let precision = new_supply.unwrap_or_else(|| old_supply.unwrap()).symbol.precision();
            let sym = Symbol::from_precision(symcode, precision);
            let supply = new_supply.unwrap_or_else(|| Asset::from_amount(0, sym));
            let supply_delta = supply.amount - old_supply.unwrap_or_else(|| Asset::from_amount(0, sym)).amount;

            let data = new_data.unwrap_or_else(|| old_data.unwrap());

            Some(SupplyChange {
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
    }).collect();


    Ok(Events { transfers, issues, retires, creates, balance_changes, supply_changes })
}

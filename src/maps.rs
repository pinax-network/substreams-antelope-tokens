use antelope::Asset;
use substreams::errors::Error;
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::abi;
use crate::balance_changes::collect_balance_changes;
use crate::eosio_token::*;
use crate::supply_changes::collect_supply_changes;
use crate::utils::{self, to_date};

#[substreams::handlers::map]
fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let transfers = block
        .actions::<abi::actions::Transfer>(&[])
        .filter_map(|(action, action_trace, trx)| {
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

                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),
            })
        })
        .collect();

    let issues = block
        .actions::<abi::actions::Issue>(&[])
        .filter_map(|(action, action_trace, trx)| {
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

                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),
            })
        })
        .collect();

    let retires = block
        .actions::<abi::actions::Retire>(&[])
        .filter_map(|(action, action_trace, trx)| {
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

                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),
            })
        })
        .collect();

    let creates = block
        .actions::<abi::actions::Create>(&[])
        .filter_map(|(action, action_trace, trx)| {
            let maximum_supply = match action.maximum_supply.parse::<Asset>() {
                Ok(asset) => asset,
                Err(e) => {
                    log::info!(
                        "Error parsing create max supply asset in trx {}: {:?}",
                        trx.id,
                        e
                    );
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

                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),
            })
        })
        .collect();

    Ok(Events {
        transfers,
        issues,
        retires,
        creates,
        balance_changes: collect_balance_changes(&clock, &block),
        supply_changes: collect_supply_changes(&clock, &block),
    })
}

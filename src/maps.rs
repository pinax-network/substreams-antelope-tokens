use antelope::{Asset, ExtendedSymbol, Name};
use substreams::errors::Error;
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::balance_changes::collect_balance_changes;
use crate::supply_changes::collect_supply_changes;
use crate::utils::{self, authorization_to_string, to_date};
use crate::{abi, Create, Events, Issue, Retire, Transfer};

#[substreams::handlers::map]
fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let transfers = block
        .actions::<abi::actions::Transfer>(&[])
        .filter_map(|(data, action_trace, trx)| {
            let quantity = match data.quantity.parse::<Asset>() {
                Ok(asset) => asset,
                Err(e) => {
                    log::info!("Error parsing transfer asset in trx {}: {:?}", trx.id, e);
                    return None;
                }
            };

            let action = action_trace.clone().action.expect("missing action");
            let contract = Name::from(action.account.as_str());
            let token = ExtendedSymbol::from_extended(quantity.symbol, contract);

            Some(Transfer {
                // block
                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),

                // transaction
                trx_id: trx.id.clone(),
                action_ordinal: action_trace.action_ordinal,
                index: action_trace.execution_index,
                authorization: authorization_to_string(&action.authorization),

                // code & scope
                contract: contract.to_string(),
                symcode: quantity.symbol.code().to_string(),
                token: token.to_string(),

                // data
                from: data.from,
                to: data.to,
                quantity: data.quantity,
                memo: data.memo,

                // extras
                precision: quantity.symbol.precision().into(),
                amount: quantity.amount,
                value: utils::to_value(&quantity),
            })
        })
        .collect();

    let issues = block
        .actions::<abi::actions::Issue>(&[])
        .filter_map(|(data, action_trace, trx)| {
            let quantity = match data.quantity.parse::<Asset>() {
                Ok(asset) => asset,
                Err(e) => {
                    log::info!("Error parsing issue asset in trx {}: {:?}", trx.id, e);
                    return None;
                }
            };
            let action = action_trace.clone().action.expect("missing action");
            let contract = Name::from(action.account.as_str());
            let token = ExtendedSymbol::from_extended(quantity.symbol, contract);

            Some(Issue {
                // block
                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),

                // transaction
                trx_id: trx.id.clone(),
                action_ordinal: action_trace.action_ordinal,
                index: action_trace.execution_index,
                authorization: authorization_to_string(&action.authorization),

                // code & scope
                contract: contract.to_string(),
                symcode: quantity.symbol.code().to_string(),
                token: token.to_string(),

                // data
                to: data.to,
                quantity: data.quantity,
                memo: data.memo,

                // extras
                precision: quantity.symbol.precision().into(),
                amount: quantity.amount,
                value: utils::to_value(&quantity),
            })
        })
        .collect();

    let retires = block
        .actions::<abi::actions::Retire>(&[])
        .filter_map(|(data, action_trace, trx)| {
            let quantity = match data.quantity.parse::<Asset>() {
                Ok(asset) => asset,
                Err(e) => {
                    log::info!("Error parsing retire asset in trx {}: {:?}", trx.id, e);
                    return None;
                }
            };

            let action = action_trace.clone().action.expect("missing action");
            let contract = Name::from(action.account.as_str());
            let token = ExtendedSymbol::from_extended(quantity.symbol, contract);

            Some(Retire {
                // block
                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),

                // transaction
                trx_id: trx.id.clone(),
                action_ordinal: action_trace.action_ordinal,
                index: action_trace.execution_index,
                authorization: authorization_to_string(&action.authorization),

                // code & scope
                contract: contract.to_string(),
                symcode: quantity.symbol.code().to_string(),
                token: token.to_string(),

                // data
                quantity: data.quantity,
                memo: data.memo,

                // extras
                precision: quantity.symbol.precision().into(),
                amount: quantity.amount,
                value: utils::to_value(&quantity),
            })
        })
        .collect();

    let creates = block
        .actions::<abi::actions::Create>(&[])
        .filter_map(|(data, action_trace, trx)| {
            let maximum_supply = match data.maximum_supply.parse::<Asset>() {
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

            let action = action_trace.clone().action.expect("missing action");
            let contract = Name::from(action.account.as_str());
            let token = ExtendedSymbol::from_extended(maximum_supply.symbol, contract);

            Some(Create {
                // block
                block_num: clock.number,
                timestamp: clock.timestamp,
                block_hash: clock.id.clone(),
                block_date: to_date(&clock),

                // transaction
                trx_id: trx.id.clone(),
                action_ordinal: action_trace.action_ordinal,
                index: action_trace.execution_index,
                authorization: authorization_to_string(&action.authorization),

                // code & scope
                contract: contract.to_string(),
                symcode: maximum_supply.symbol.code().to_string(),
                token: token.to_string(),

                // data
                issuer: data.issuer,
                maximum_supply: data.maximum_supply,

                // extras
                precision: maximum_supply.symbol.precision().into(),
                amount: maximum_supply.amount,
                value: utils::to_value(&maximum_supply),
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

use antelope::{Asset, ExtendedSymbol, Name};
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::utils::{self, authorization_to_string, to_date};
use crate::{abi, Retire};

pub fn collect_retires(clock: &Clock, block: &Block) -> Vec<Retire> {
    block
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
                timestamp: clock.timestamp,
                block_num: clock.number,
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
        .collect()
}

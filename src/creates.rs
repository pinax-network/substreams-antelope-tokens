use antelope::{Asset, ExtendedSymbol, Name};
use substreams::log;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::utils::{self, authorization_to_string, to_date};
use crate::{abi, Create};

pub fn collect_creates(clock: &Clock, block: &Block) -> Vec<Create> {
    block
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
        .collect()
}

use crate::{abi, SupplyChange};
use antelope::{Asset, Name, Symbol, SymbolCode};
use substreams::{log, pb::substreams::Clock};
use substreams_antelope::decoder::decode;
use substreams_antelope::Block;

use crate::utils;

pub fn collect_supply_changes(clock: &Clock, block: &Block) -> Vec<SupplyChange> {
    block
        .transaction_traces()
        .flat_map(|trx| {
            trx.db_ops.iter().filter_map(|db_op| {
                if db_op.table_name != "stat" {
                    return None;
                }

                let old_data = decode::<abi::types::CurrencyStats>(&db_op.old_data_json).ok();
                let new_data = decode::<abi::types::CurrencyStats>(&db_op.new_data_json).ok();

                let old_supply =
                    old_data
                        .as_ref()
                        .and_then(|data| match data.supply.parse::<Asset>() {
                            Ok(asset) => Some(asset),
                            Err(e) => {
                                log::info!(
                                    "Error parsing old supply asset in trx {}: {:?}",
                                    trx.id,
                                    e
                                );
                                None
                            }
                        });

                let new_supply =
                    new_data
                        .as_ref()
                        .and_then(|data| match data.supply.parse::<Asset>() {
                            Ok(asset) => Some(asset),
                            Err(e) => {
                                log::info!(
                                    "Error parsing new supply asset in trx {}: {:?}",
                                    trx.id,
                                    e
                                );
                                None
                            }
                        });

                if old_supply.is_none() && new_supply.is_none() {
                    return None;
                }

                let symcode = SymbolCode::from(Name::from(db_op.primary_key.as_str()).value);
                let precision = new_supply
                    .unwrap_or_else(|| old_supply.unwrap())
                    .symbol
                    .precision();
                let sym = Symbol::from_precision(symcode, precision);
                let supply = new_supply.unwrap_or_else(|| Asset::from_amount(0, sym));
                let supply_delta = supply.amount
                    - old_supply
                        .unwrap_or_else(|| Asset::from_amount(0, sym))
                        .amount;

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

                    block_num: clock.number,
                    timestamp: clock.timestamp,
                    block_hash: clock.id.clone(),
                    block_date: utils::to_date(&clock),
                })
            })
        })
        .collect()
}

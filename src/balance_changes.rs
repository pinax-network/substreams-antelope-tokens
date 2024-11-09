use crate::{abi, BalanceChange};
use antelope::{Asset, Name, Symbol, SymbolCode};
use substreams::{log, pb::substreams::Clock};
use substreams_antelope::decoder::decode;
use substreams_antelope::Block;

use crate::utils;

pub fn collect_balance_changes(clock: &Clock, block: &Block) -> Vec<BalanceChange> {
    block
        .transaction_traces()
        .flat_map(|trx| {
            trx.db_ops.iter().filter_map(|db_op| {
                if db_op.table_name != "accounts" {
                    return None;
                }

                let old_data = decode::<abi::types::Account>(&db_op.old_data_json).ok();
                let new_data = decode::<abi::types::Account>(&db_op.new_data_json).ok();

                let old_balance =
                    old_data
                        .as_ref()
                        .and_then(|data| match data.balance.parse::<Asset>() {
                            Ok(asset) => Some(asset),
                            Err(e) => {
                                log::info!(
                                    "Error parsing old balance asset in trx {}: {:?}",
                                    trx.id,
                                    e
                                );
                                None
                            }
                        });
                let new_balance =
                    new_data
                        .as_ref()
                        .and_then(|data| match data.balance.parse::<Asset>() {
                            Ok(asset) => Some(asset),
                            Err(e) => {
                                log::info!(
                                    "Error parsing new balance asset in trx {}: {:?}",
                                    trx.id,
                                    e
                                );
                                None
                            }
                        });

                if old_balance.is_none() && new_balance.is_none() {
                    return None;
                }

                let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
                let symcode = SymbolCode::from(raw_primary_key);
                let precision = new_balance
                    .unwrap_or_else(|| old_balance.unwrap())
                    .symbol
                    .precision();
                let sym = Symbol::from_precision(symcode, precision);
                let balance = new_balance.unwrap_or_else(|| Asset::from_amount(0, sym));
                let balance_delta = balance.amount
                    - old_balance
                        .unwrap_or_else(|| Asset::from_amount(0, sym))
                        .amount;

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

                    block_num: clock.number,
                    timestamp: clock.timestamp,
                    block_hash: clock.id.clone(),
                    block_date: utils::to_date(&clock),
                })
            })
        })
        .collect()
}

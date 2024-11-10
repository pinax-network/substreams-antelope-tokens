use crate::{
    utils::{parse_json_name, to_date},
    SupplyChange,
};
use antelope::{Asset, ExtendedSymbol, Name};
use substreams::{log, pb::substreams::Clock};
use substreams_antelope::Block;

use crate::utils::{self, parse_json_asset};

pub fn collect_supply_changes(clock: &Clock, block: &Block) -> Vec<SupplyChange> {
    let mut index = 0; // incremental index for each supply change
    block
        .transaction_traces()
        .flat_map(|trx| {
            trx.db_ops.iter().filter_map(move |db_op| {
                if db_op.table_name != "stat" {
                    return None;
                }
                // token contract
                let contract = Name::from(db_op.code.as_str());

                // ignore invalid contract or account
                if contract.value == 0 {
                    log::info!("Invalid contract in trx {}: contract: {}", trx.id, contract,);
                    return None;
                }

                // parse Assets
                let old_supply = parse_json_asset(&db_op.old_data_json, "supply");
                let new_supply = parse_json_asset(&db_op.new_data_json, "supply");
                let old_max_supply = parse_json_asset(&db_op.old_data_json, "max_supply");
                let new_max_supply = parse_json_asset(&db_op.new_data_json, "max_supply");
                let new_issuer = parse_json_name(&db_op.new_data_json, "issuer");

                // no valid Assets
                if old_supply.is_none() && new_supply.is_none() {
                    return None;
                }

                // ignore mismatched supply
                if old_supply.is_some() && new_supply.is_some() {
                    if old_supply.unwrap().symbol != new_supply.unwrap().symbol {
                        log::info!(
                            "Mismatched supply in trx {}: old_supply: {:?}, new_supply: {:?}",
                            trx.id,
                            old_supply,
                            new_supply
                        );
                        return None;
                    }
                }

                // fields derived from old_balance or new_balance
                let sym = old_supply
                    .or(new_supply)
                    .as_ref()
                    .expect("missing old_supply or new_supply")
                    .symbol;
                let token = ExtendedSymbol::from_extended(sym, contract);
                let zero = Asset::from_amount(0, sym);
                let issuer = new_issuer.unwrap_or(Name::new());

                // supply
                let old_supply = old_supply.as_ref().unwrap_or(&zero);
                let supply = new_supply.as_ref().unwrap_or(&zero);
                let supply_delta = supply.amount - old_supply.amount;

                // max supply
                let old_max_supply = old_max_supply.as_ref().unwrap_or(&zero);
                let max_supply = new_max_supply.as_ref().unwrap_or(&zero);
                let max_supply_delta = max_supply.amount - old_max_supply.amount;
                index += 1;

                Some(SupplyChange {
                    // block
                    timestamp: clock.timestamp,
                    block_num: clock.number,
                    block_hash: clock.id.clone(),
                    block_date: to_date(&clock),

                    // transaction
                    trx_id: trx.id.clone(),
                    action_index: db_op.action_index,
                    operation: db_op.operation().as_str_name().to_string(),
                    index,

                    // code & scope
                    contract: contract.to_string(),
                    symcode: supply.symbol.code().to_string(),
                    token: token.to_string(),

                    // data
                    issuer: issuer.to_string(),
                    supply: supply.to_string(),
                    supply_delta,
                    max_supply: max_supply.to_string(),
                    max_supply_delta,

                    // extras
                    precision: sym.precision().into(),
                    amount: supply.amount,
                    value: utils::to_value(&supply),
                })
            })
        })
        .collect()
}

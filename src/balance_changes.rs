use crate::{utils::to_date, BalanceChange};
use antelope::{Asset, ExtendedSymbol, Name};
use substreams::{log, pb::substreams::Clock};
use substreams_antelope::Block;

use crate::utils::{self, parse_json_asset};

pub fn collect_balance_changes(clock: &Clock, block: &Block) -> Vec<BalanceChange> {
    let mut index = 0; // incremental index for each balance change
    block
        .transaction_traces()
        .flat_map(|trx| {
            trx.db_ops.iter().filter_map(move |db_op| {
                if db_op.table_name != "accounts" {
                    return None;
                }

                // decoded
                let old_balance = parse_json_asset(&db_op.old_data_json, "balance");
                let new_balance = parse_json_asset(&db_op.new_data_json, "balance");

                // no valid Accounts
                if old_balance.is_none() && new_balance.is_none() {
                    return None;
                }

                // token contract & account
                let contract = Name::from(db_op.code.as_str());
                let account = Name::from(db_op.scope.as_str());

                // ignore invalid contract or account
                if contract.value == 0 || account.value == 0 {
                    log::info!(
                        "Invalid contract or account in trx {}: contract: {}, account: {}",
                        trx.id,
                        contract,
                        account
                    );
                    return None;
                }

                // ignore mismatched balances
                if old_balance.is_some() && new_balance.is_some() {
                    if old_balance.unwrap().symbol != new_balance.unwrap().symbol {
                        log::info!(
                            "Mismatched balance in trx {}: old_balance: {:?}, new_balance: {:?}",
                            trx.id,
                            old_balance,
                            new_balance
                        );
                        return None;
                    }
                }

                // fields derived from old_balance or new_balance
                let sym = old_balance
                    .or(new_balance)
                    .as_ref()
                    .expect("missing old_balance or new_balance")
                    .symbol;
                let token = ExtendedSymbol::from_extended(sym, contract);
                let zero = Asset::from_amount(0, sym);
                let balance = new_balance.as_ref().unwrap_or(&zero);
                let old_balance = old_balance.as_ref().unwrap_or(&zero);
                let balance_delta = balance.amount - old_balance.amount;
                index += 1;

                Some(BalanceChange {
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
                    symcode: balance.symbol.code().to_string(),
                    token: token.to_string(),

                    // data
                    account: account.to_string(),
                    balance: balance.to_string(),
                    balance_delta,

                    // extras
                    precision: sym.precision().into(),
                    amount: balance.amount,
                    value: utils::to_value(&balance),
                })
            })
        })
        .collect()
}

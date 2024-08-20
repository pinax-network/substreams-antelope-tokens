use crate::keys::token_key;
use crate::pb::antelope::eosio::token::v1::{Events, Token};
use substreams::prelude::*;
use substreams::store::StoreSetProto;

#[substreams::handlers::store]
pub fn store_tokens(events: Events, s: StoreSetProto<Token>) {
    for event in events.creates {
        let token = Token {
            contract: event.contract.clone(),
            symcode: event.symcode.clone(),
            precision: event.precision,
            issuer: event.issuer,
            max_supply: event.maximum_supply,
            created_blocknum: event.block_num,
            created_tx: event.trx_id,
        };

        s.set(
            event.action_index as u64,
            token_key(&event.contract, &event.symcode),
            &token,
        );
    }
}

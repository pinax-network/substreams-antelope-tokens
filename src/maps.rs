use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;

use crate::balance_changes::collect_balance_changes;
use crate::creates::collect_creates;
use crate::issues::collect_issues;
use crate::retires::collect_retires;
use crate::supply_changes::collect_supply_changes;
use crate::transfers::collect_transfers;
use crate::Events;

#[substreams::handlers::map]
fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    Ok(Events {
        transfers: collect_transfers(&clock, &block),
        issues: collect_issues(&clock, &block),
        retires: collect_retires(&clock, &block),
        creates: collect_creates(&clock, &block),
        balance_changes: collect_balance_changes(&clock, &block),
        supply_changes: collect_supply_changes(&clock, &block),
    })
}

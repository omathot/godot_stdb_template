use log::{log, Level};
use spacetimedb::{reducer, table, Identity, ReducerContext, SpacetimeType, Table, Timestamp};
mod math;
mod player;

use math::DbVec2;

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) -> Result<(), String> {
    log!(Level::Info, "Initializing...");

    Ok(())
}

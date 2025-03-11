use log::{log, Level};
use spacetimedb::{reducer, table, Identity, ReducerContext, SpacetimeType, Table, Timestamp};
mod math;
mod player;

use math::DbVec2;
use player::Player;

#[table(name=config, public)]
pub struct Config {
    #[primary_key]
    pub id: u32,
    pub world_size: u64,
}

#[reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    log!(Level::Info, "Initializing...");
    ctx.db.config().try_insert(Config {
        id: 0,
        world_size: 1000,
    })?;

    Ok(())
}

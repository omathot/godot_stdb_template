use super::*;
use log::{log, Level};
use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

// public means READABLE BY CLIENT, reducers can still access non-public tables. PUBLIC FOR CONNECTED CLIENTS
#[table(name=entity, public)]
#[derive(Debug, Clone)]
pub struct Entity {
    #[auto_inc]
    #[primary_key]
    pub entity_id: u32,
    pub position: DbVec2,
    pub mass: u32,
}

#[table(name=player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity, // Identity represents a user connecting to database, NO RELATION TO ECS ENTITY
    #[unique]
    #[auto_inc] // will automatically increment 0 value on init
    player_id: u32, // Entity id (relational database)
    username: Option<String>,
    online: bool,
    last_connection: Timestamp,
}

// still need the struct that builds the relation between player and entity like Circle in example

#[reducer(client_connected)]
pub fn player_connected(ctx: &ReducerContext) {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        log!(
            Level::Debug,
            "Setting {:?}(identity: {}) to online",
            player.username,
            player.identity
        );
        ctx.db.player().identity().update(Player {
            online: true,
            last_connection: ctx.timestamp,
            ..player
        });
    } else {
        log!(Level::Debug, "Creating new Player Identity: {}", ctx.sender);
        ctx.db.player().insert(Player {
            identity: ctx.sender,
            player_id: 0,
            username: None,
            online: true,
            last_connection: ctx.timestamp,
        });
    }
}

#[reducer(client_disconnected)]
pub fn player_disconnected(ctx: &ReducerContext) {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        log!(
            Level::Debug,
            "{:?} disconnected (Identity: {})",
            player.username,
            player.identity
        );
        ctx.db.player().identity().update(Player {
            online: false,
            last_connection: ctx.timestamp,
            ..player
        });
    } else {
        log!(
            Level::Warn,
            "Disconnection event for unknown Identity: {}",
            ctx.sender
        );
    }
}

#[reducer]
pub fn set_player_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(&name)?;
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        if player.username != None {
            log!(
                Level::Debug,
                "Overwriting username {:?} to {:?}",
                player.username,
                name
            );
        } else {
            log!(
                Level::Debug,
                "Setting Identity {} username to {:?}",
                player.identity,
                name
            );
        }
        ctx.db.player().identity().update(Player {
            username: Some(name),
            ..player
        });
        Ok(())
    } else {
        log!(
            Level::Error,
            "Cannot set name because Identity {} doesn't exist as Player",
            ctx.sender
        );
        Err("Cannot set name for unknown player".to_string())
    }
}

fn validate_name(name: &String) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name can't be empty".to_string());
    } else {
        Ok(name.clone())
    }
}

//---------------------------------------------------------------------------------------------------
// ------------------------------TESTING---------------------------

#[reducer]
pub fn insert_mock_player(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let player = ctx.db.player().try_insert(Player {
        identity: ctx.sender,
        player_id: 0,
        username: Some(name),
        online: true,
        last_connection: ctx.timestamp,
    })?;
    log!(
        Level::Info,
        "Successfully inserted mock player with identity: {}",
        player.identity
    );

    Ok(())
}

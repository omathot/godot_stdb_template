mod game_manager;
mod module_bindings;

use godot::classes::{
    ConeTwistJoint3D, Engine, ISprite2D, Sprite2D, VisualShaderNodeParticleOutput,
};
use godot::prelude::*;
use module_bindings::*;
use spacetimedb_sdk::{DbContext, Error, Identity, Table, TableWithPrimaryKey, credentials};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

/*
    architecture:
    DbManager (Connection Layer)
        ↓
    Game Managers (Logic Layer)
        ↓
    Game Resources (Data Layer)
*/

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "test";

// Extension boilerplate --
struct MyExtenion;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtenion {}
// --

// Helper
fn creds_store() -> credentials::File {
    credentials::File::new("test")
}

fn setup_subscriptions(ctx: &SubscriptionEventContext) {
    godot_print!("Setting up table subscriptions");

    let _player_handle = ctx
        .subscription_builder()
        .on_applied(|_| godot_print!("Player table subscription applied"))
        .on_error(|_, e| godot_error!("Player table subscription error: {:?}", e))
        .subscribe("SELECT * FROM player");

    let _entity_handle = ctx
        .subscription_builder()
        .on_applied(|_| godot_print!("Entity table subscriptions applied"))
        .on_error(|_, e| godot_error!("Entity table subscription error: {:?}", e))
        .subscribe("SELECT * FROM entity");

    register_table_callbacks(ctx);
}

fn register_table_callbacks(ctx: &SubscriptionEventContext) {
    ctx.db.player().on_insert(|_, row| {
        godot_print!(
            "Player inserted: {}",
            row.username.as_ref().unwrap_or(&"unnamed".to_string())
        )
    });
    ctx.db.player().on_update(|_, row, old| {
        godot_print!(
            "Player updated {} -> {}",
            old.username.as_ref().unwrap_or(&"unnamed".to_string()),
            row.username.as_ref().unwrap_or(&"unnamed".to_string())
        )
    });
    ctx.db.player().on_delete(|_, row| {
        godot_print!(
            "Player deleted: {}",
            row.username.as_ref().unwrap_or(&"unnamed".to_string())
        )
    });

    ctx.db
        .entity()
        .on_insert(|_, row| godot_print!("Entity inserted: {}", row.entity_id));

    ctx.db.entity().on_update(|_, row, old| {
        godot_print!("Entity updated {} -> {}", old.entity_id, row.entity_id)
    });
    ctx.db
        .entity()
        .on_delete(|_, row| godot_print!("Entity deleted: {}", row.entity_id));
}

// Connection callbacks
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

fn on_disconnected(_ctx: &ErrorContext, error: Option<Error>) {
    if let Some(err) = error {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected");
        std::process::exit(0);
    }
}

// Table Subsciptions
fn on_player_insert(event: &EventContext, row: &Player) {
    godot_print!("Player inserted: {:?}", row.username);
}

fn on_player_update(event: &EventContext, row: &Player, old: &Player) {
    godot_print!("Player updated: {:?} -> {:?}", row.username, old.username);
}

fn on_player_delete(event: &EventContext, row: &Player) {
    godot_print!("Player deleted: {:?}", row.username);
}

fn on_entity_insert(event: &EventContext, row: &Entity) {
    godot_print!("Entity inserted: {}", row.entity_id);
}

fn on_entity_update(event: &EventContext, row: &Entity, old: &Entity) {
    godot_print!("Entity updated: {}", row.entity_id);
}

fn on_entity_delete(event: &EventContext, row: &Entity) {
    godot_print!("Entity deleted: {}", row.entity_id);
}

#[derive(GodotClass)]
#[class(tool, base=Node)]
pub struct DbManager {
    connection: Option<DbConnection>,
    identity: Option<Identity>,
    runtime: Option<Runtime>,
    player_subscription: Option<SubscriptionHandle>,
    entity_subscription: Option<SubscriptionHandle>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for DbManager {
    fn init(base: Base<Node>) -> DbManager {
        godot_print!("Initializing DbManager!");
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .ok();
        if runtime.is_none() {
            godot_error!("Failed to create tokio runtime");
        }
        DbManager {
            connection: None,
            identity: None,
            runtime,
            player_subscription: None,
            entity_subscription: None,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("DbManager is ready!");
        // can auto start
        // self.connect_to_db();
    }

    fn process(&mut self, _delta: f64) {
        if let Some(connection) = &self.connection {
            if let Err(e) = connection.frame_tick() {
                godot_error!("Error processing SpaceTimeDB messages: {:?}", e);
            }
        }
    }
}

#[godot_api]
impl DbManager {
    #[func]
    pub fn get_instance() -> Variant {
        if let Some(singleton) = Engine::singleton().get_singleton("GlobalDbManager") {
            match singleton.try_cast::<DbManager>() {
                Ok(manager) => return manager.to_variant(),
                Err(_) => return Variant::nil(),
            }
        } else {
            Variant::nil()
        }
    }

    #[func]
    pub fn connect_to_db(&mut self) -> bool {
        godot_print!("Connecting to SpaceTimeDB at: {}/{}", HOST, DB_NAME);

        match DbConnection::builder()
            .on_connect(on_connected)
            .on_connect_error(on_connect_error)
            .on_disconnect(on_disconnected)
            .with_token(creds_store().load().expect("Error loading credentials"))
            .with_module_name(DB_NAME)
            .with_uri(HOST)
            .build()
        {
            Ok(connection) => {
                connection.run_threaded();
                self.connection = Some(connection);
                true
            }
            Err(e) => {
                godot_error!("Failed to connect {:?}", e);
                false
            }
        }
    }

    #[func]
    pub fn disconnect_from_db(&mut self) -> bool {
        if let Some(connection) = &self.connection {
            if let Err(e) = connection.disconnect() {
                godot_error!("Error disconnecting: {:?}", e);
                return false;
            }
            self.connection = None;
            true
        } else {
            godot_error!("Not connected to database!");
            false
        }
    }

    #[func]
    pub fn is_connected(&self) -> bool {
        if let Some(connection) = &self.connection {
            connection.is_active()
        } else {
            false
        }
    }

    #[func]
    pub fn call_reducer(&self, name: GString, args: Array<Variant>) -> bool {
        if let Some(connection) = &self.connection {
            if let Some(rt) = &self.runtime {
                let reducer_name = name.to_string();
                let result = match reducer_name.as_str() {
                    "insert_mock_player" => {
                        if args.len() >= 1 {
                            let player_name = args.get(0).unwrap().to_string();
                            connection.reducers.insert_mock_player(player_name)
                        } else {
                            godot_error!("insert_mock_player requires a name argument");
                            return false;
                        }
                    }
                    "player_connected" => connection.reducers.player_connected(),
                    "player_disconnected" => connection.reducers.player_disconnected(),
                    "set_player_name" => {
                        if args.len() >= 1 {
                            let player_name = args.get(0).unwrap().to_string();
                            connection.reducers.set_player_name(player_name)
                        } else {
                            godot_error!("set_player_name requires a name argument!");
                            return false;
                        }
                    }
                    _ => {
                        godot_error!("Unknown reducer!");
                        return false;
                    }
                };

                match result {
                    Ok(_) => true,
                    Err(e) => {
                        godot_error!("Failed to call reducer {}: {:?}", name, e);
                        false
                    }
                }
            } else {
                godot_error!("Runtime not available!");
                false
            }
        } else {
            godot_error!("Not connected to database!");
            false
        }
    }

    #[func]
    pub fn get_identity(&self) -> GString {
        if let Some(identity) = &self.identity {
            GString::from(identity.to_string())
        } else if let Some(connection) = &self.connection {
            if let Some(identity) = connection.try_identity() {
                GString::from(identity.to_string())
            } else {
                GString::from("No identity")
            }
        } else {
            GString::from("Not connected")
        }
    }

    #[func]
    pub fn get_all_players(&self) -> Dictionary {
        let mut result = Dictionary::new();

        if let Some(connection) = &self.connection {
            let player_table = connection.db.player(); // necessary middle step for Rust memory safety
            let players = player_table.iter();
            for (i, player) in players.enumerate() {
                let mut player_data = Dictionary::new();
                player_data.set("identity", player.identity.to_string().to_variant());
                player_data.set(
                    "name",
                    player
                        .username
                        .as_ref()
                        .unwrap_or(&"unnamed".to_string())
                        .to_variant(),
                );

                let num: i32 = i as i32;
                result.set(num.to_variant(), player_data.to_variant());
            }
        }
        result
    }

    #[func]
    pub fn get_all_entities(&self) -> Dictionary {
        let mut result = Dictionary::new();

        if let Some(connection) = &self.connection {
            let entity_table = connection.db.entity();
            let entities = entity_table.iter();
            for (i, entity) in entities.enumerate() {
                let mut entity_data = Dictionary::new();
                entity_data.set("entity_id", entity.entity_id);

                let num: i32 = i as i32;
                result.set(num.to_variant(), entity_data.to_variant());
            }
        }

        result
    }

    #[func]
    pub fn get_connection(&self) -> Variant {
        if let Some(_) = self.connection {
            Variant::nil() // placeholder until i figure out what to return here
        } else {
            Variant::nil()
        }
    }
}

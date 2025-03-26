mod godot_modules;
mod module_bindings;

use godot::prelude::*;
use godot_modules::PlayerModule;
use module_bindings::*;
use spacetimedb_sdk::{DbContext, Error, Identity, Table, TableWithPrimaryKey, credentials};

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

#[derive(GodotClass)]
#[class(tool, base=Node)]
pub struct DbManager {
    connection: Option<DbConnection>,
    player_module: Option<Gd<PlayerModule>>,
    // entity_module: Option<Gd<EntityModule>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for DbManager {
    fn init(base: Base<Node>) -> DbManager {
        godot_print!("Initializing DbManager!");

        let player_module = PlayerModule::new_alloc();
        let manager = DbManager {
            connection: None,
            player_module: Some(player_module),
            base,
        };

        manager
    }

    fn ready(&mut self) {
        godot_print!("DbManager is ready!");
        if let Some(player_mod) = &mut self.player_module {
            player_mod.set_name("PlayerModule");
        }
        // add as children for lifecyle (testing this)
        if let Some(player_mod) = &self.player_module {
            let player_node = player_mod.clone().upcast::<Node>();
            self.base_mut().add_child(&player_node);
        }

        self.connect_modules();
    }

    fn process(&mut self, _delta: f64) {
        if let Some(connection) = &self.connection {
            if let Err(e) = connection.frame_tick() {
                godot_error!("Error processing during SpaceTimeDB frame_tick(): {:?}", e);
            }
        }
    }
}

#[godot_api]
impl DbManager {
    #[func]
    fn connect_modules(&mut self) {
        let path_str = self.base().get_path().to_string();
        let path = GString::from(path_str);
        godot_print!("DbManager path: {}", path);

        if let Some(player_mod) = &mut self.player_module {
            player_mod.call("set_connection_parent_path", &[path.clone().to_variant()]);
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
                // redudant, frame_tick() in process(), also run_threaded not as good for games
                // connection.run_threaded();
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

    // #[func]
    // fn get_connection(&self) -> Option<&DbConnection> {
    //     self.connection.as_ref()
    // }

    #[signal]
    fn connection_state_changed(connected: bool);
}

impl Drop for DbManager {
    fn drop(&mut self) {
        if let Some(connection) = &self.connection {
            if let Err(e) = connection.disconnect() {
                godot_error!("Error disconnecting: {:?}", e);
            }
            self.connection = None;
        } else {
            godot_error!("Not connected to database!");
        }
    }
}

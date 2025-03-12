use crate::module_bindings::*;
use godot::prelude::*;
use spacetimedb_sdk::{DbContext, Identity};
use std::sync::Arc;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerModule {
    connection: Option<Gd<Node>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerModule {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Initializing Player Module!");
        PlayerModule {
            connection: None,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("PlayerModule ready!");
    }
}

#[godot_api]
impl PlayerModule {
    // Will be called by DbManager
    #[func]
    pub fn set_connection_parent_path(&mut self, path: GString) {
        let node_path = NodePath::from(path);
        if let Some(parent_node) = self
            .base()
            .get_tree()
            .and_then(|tree| tree.get_root())
            .and_then(|root| root.try_get_node_as::<Node>(&node_path))
        {
            self.connection = Some(parent_node);
            godot_print!("PlayerModule: Connection setup via parent path");
        }
    }

    // // deprecate already lol
    // #[func]
    // pub fn set_connection_parent(&mut self, parent: Gd<Node>) {
    //     self.connection = Some(parent);
    //     godot_print!("PlayerModule: Connection parent set!");
    // }

    // private helper
    fn get_db_connection(&self) -> Option<Gd<Node>> {
        if let Some(parent) = &self.connection {
            return Some(parent.clone());
        }
        None
    }

    #[func]
    pub fn get_connection_status(&self) -> bool {
        if let Some(mut parent) = self.get_db_connection() {
            let result = parent.call("is_connected", &[]);
            if let Ok(is_connected) = result.try_to::<bool>() {
                return is_connected;
            }
        }
        false
    }

    #[func]
    pub fn set_player_name(&self, name: GString) -> bool {
        if let Some(mut parent) = self.get_db_connection() {
            let args = name.to_variant();
            let result = parent.call(
                "call_reducer",
                &[
                    GString::from("set_player_name").to_variant(),
                    args.to_variant(),
                ],
            );
            if let Ok(success) = result.try_to::<bool>() {
                godot_print!("Set player name to: {}", name);
                return success;
            }
        }
        godot_error!("Cannot set playing name: No connection");
        false
    }

    #[func]
    pub fn get_my_player(&self) -> Dictionary {
        if let Some(mut parent) = self.get_db_connection() {
            let identity_result = parent.call("get_identity", &[]);
            if let Ok(identity) = identity_result.try_to::<GString>() {
                // get all and find by identity
                let all_players_result = self.get_all_players();

                for (_, player_variant) in all_players_result.iter_shared() {
                    if let Ok(player_data) = player_variant.try_to::<Dictionary>() {
                        if let Some(identity_variant) = player_data.get("identity") {
                            if let Ok(player_identity) = identity_variant.try_to::<GString>() {
                                if player_identity == identity {
                                    return player_data;
                                }
                            }
                        }
                    }
                }
            }
        }
        Dictionary::new()
    }

    #[func]
    pub fn get_all_players(&self) -> Dictionary {
        let result = Dictionary::new();

        if let Some(mut parent) = self.get_db_connection() {
            let player_result = parent.call("get_all_players", &[]);
            if let Ok(players) = player_result.try_to::<Dictionary>() {
                return players;
            }
        }
        result
    }

    #[signal]
    fn player_joined(player_id: i32, player_name: GString);
    #[signal]
    fn player_left(player_id: i32);
    #[signal]
    fn player_renamed(player_id: i32, new_name: GString);
}

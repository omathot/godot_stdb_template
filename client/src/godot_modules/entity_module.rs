use crate::module_bindings::*;
use godot::prelude::*;
use spacetimedb_sdk::DbContext;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct EntityModule {
    connection: Option<Gd<Node>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for EntityModule {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Initializing EntitiyModule!");
        EntityModule {
            connection: None,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("EntityModule ready");
    }
}

#[godot_api]
impl EntityModule {
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
            godot_print!("EntityModule: Connection setup via parent path");
        }
    }

    // deprecate
    #[func]
    pub fn set_connection_parent(&mut self, parent: Gd<Node>) {
        self.connection = Some(parent);
        godot_print!("EntityModule: Connection parent set");
    }

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
            if let Ok(success) = result.try_to::<bool>() {
                return success;
            }
        }
        false
    }

    #[func]
    pub fn get_entity_by_id(&self, entity_id: u32) -> Dictionary {
        if let Some(mut parent) = self.get_db_connection() {
            let entities_result = parent.call("get_all_entities", &[]);
            if let Ok(entities) = entities_result.try_to::<Dictionary>() {
                // search through entities and match ID
                for (_, entity_variant) in entities.iter_shared() {
                    if let Ok(entity_data) = entity_variant.try_to::<Dictionary>() {
                        if let Some(id_variant) = entity_data.get("entity_id") {
                            if let Ok(id) = id_variant.try_to::<u32>() {
                                if id == entity_id {
                                    return entity_data;
                                }
                            }
                        }
                    }
                }
            }
        }
        // didn't find, empty
        Dictionary::new()
    }

    #[func]
    pub fn get_all_entities(&self) -> Dictionary {
        if let Some(mut parent) = self.get_db_connection() {
            let entities_result = parent.call("get_all_entities", &[]);
            if let Ok(entities) = entities_result.try_to::<Dictionary>() {
                return entities;
            }
        }
        Dictionary::new()
    }

    #[signal]
    fn entity_created(entity_id: i32);
    #[signal]
    fn entity_updated(entity_id: i32);
    #[signal]
    fn entity_removed(entity_id: i32);
}

// use super::*;
// use godot::{classes::ResourceLoader, prelude::*};

// #[derive(GodotClass)]
// #[class(base=Node)]
// pub struct GameManager {
//     player_scene: Option<Gd<PackedScene>>,
//     entity_scene: Option<Gd<PackedScene>>,
//     entities: Dictionary,
//     players: Dictionary,
//     local_player_id: GString,
//     db_manager: Option<Gd<Node>>,
//     #[base]
//     base: Base<Node>,
// }

// #[godot_api]
// impl INode for GameManager {
//     fn init(base: Base<Node>) -> Self {
//         godot_print!("Initializing game manager!");

//         Self {
//             player_scene: None,
//             entity_scene: None,
//             entities: Dictionary::new(),
//             players: Dictionary::new(),
//             local_player_id: GString::new(),
//             db_manager: None,
//             base,
//         }
//     }

//     fn ready(&mut self) {
//         godot_print!("GameManager is ready!");

//         self.db_manager = None;
//         // self.db_manager = Engine::singleton()
//         //     .get_singleton("GlobalDbManager")
//         //     .and_then(|v| v.try_cast::<Node>().ok());

//         let mut resource_loader = ResourceLoader::singleton();
//         self.player_scene = resource_loader
//             .load("res://scenes/player.tscn")
//             .map(|res| res.try_cast::<PackedScene>().ok())
//             .flatten();

//         self.entity_scene = resource_loader
//             .load("res://scenes/entity.tscn")
//             .map(|res| res.try_cast::<PackedScene>().ok())
//             .flatten();

//         if self.player_scene.is_none() {
//             godot_error!("Failed to load player scene!");
//         }
//         if self.entity_scene.is_none() {
//             godot_error!("Failed to load entity scene!");
//         }
//     }

//     fn process(&mut self, _delta: f64) {
//         self.update_game_state();
//     }
// }

// #[godot_api]
// impl GameManager {
//     #[func]
//     pub fn connect_to_db(&mut self) -> bool {
//         godot_print!("GameManager: Connecting to database");

//         if let Some(db_manager) = &mut self.db_manager {
//             let result = db_manager.call("connect_to_db", &[]);
//             if let Ok(success) = result.try_to::<bool>() {
//                 return success;
//             }
//         }
//         false
//     }

//     #[func]
//     pub fn join_game(&mut self, player_name: GString) -> bool {
//         if let Some(db_manager) = &mut self.db_manager {
//             let args = [player_name.to_variant()];
//             let result = db_manager.call(
//                 "call_reducer",
//                 &[
//                     GString::from("set_player_name").to_variant(),
//                     args.to_variant(),
//                 ],
//             );
//             if let Ok(success) = result.try_to::<bool>() {
//                 return success;
//             }
//         }
//         false
//     }

//     #[func]
//     pub fn update_game_state(&mut self) {
//         if !self.is_connected() {
//             return;
//         }

//         let players_data = self.get_all_players();
//         let entity_data = self.get_all_entities();

//         self.update_player_entities(players_data);
//         self.update_game_entities(entity_data);
//     }

//     #[func]
//     pub fn is_connected(&mut self) -> bool {
//         if let Some(db_manager) = &mut self.db_manager {
//             let result = db_manager.call("is_connected", &[]);
//             if let Ok(success) = result.try_to::<bool>() {
//                 return success;
//             }
//         }
//         false
//     }

//     #[func]
//     fn get_all_players(&mut self) -> Dictionary {
//         if let Some(db_manager) = &mut self.db_manager {
//             let result = db_manager.call("get_all_players", &[]);
//             if let Ok(players) = result.try_to::<Dictionary>() {
//                 return players;
//             }
//         }
//         Dictionary::new()
//     }

//     #[func]
//     fn get_all_entities(&mut self) -> Dictionary {
//         if let Some(db_manager) = &mut self.db_manager {
//             let result = db_manager.call("get_all_entities", &[]);
//             if let Ok(entities) = result.try_to::<Dictionary>() {
//                 return entities;
//             }
//         }
//         Dictionary::new()
//     }

//     // this is about maintaing the godot tree structure, not related to DataBase Operations
//     #[func]
//     fn update_player_entities(&mut self, players_data: Dictionary) {
//         let mut players_to_keep = Dictionary::new();
//         for (_key, player_variant) in players_data.iter_shared() {
//             if let Ok(player_data) = player_variant.try_to::<Dictionary>() {
//                 let player_id = player_data.at("identity".to_string());
//                 let name = player_data.at("username".to_string());

//                 if !self.players.contains_key(player_id.clone()) {
//                     if let Some(player_scene) = &self.player_scene {
//                         if let Some(instance) = player_scene.instantiate() {
//                             if let Ok(mut sprite) = instance.try_cast::<Sprite2D>() {
//                                 sprite.set("player_name", &name.to_variant());
//                                 sprite.set("player_id", &player_id.to_variant());

//                                 self.base_mut().add_child(&sprite.clone().upcast::<Node>());
//                                 self.players.set(player_id.clone(), sprite.to_variant());
//                                 players_to_keep.set(player_id.clone(), sprite.to_variant());
//                                 godot_print!("Created player: {}, ({})", name, player_id);
//                             }
//                         }
//                     }
//                 }
//                 if let Some(player_node) = self.players.get(player_id.clone()) {
//                     players_to_keep.set(player_id.clone(), player_node);
//                 }
//             }
//         }

//         for (key, player_node_variant) in self.players.iter_shared() {
//             let player_id = key.to_string();
//             if !players_to_keep.contains_key(player_id.clone()) {
//                 if let Ok(mut player_node) = player_node_variant.try_to::<Gd<Node>>() {
//                     player_node.queue_free();
//                     godot_print!("Removed player: {}", player_id);
//                 }
//             }
//         }

//         self.players = players_to_keep;
//     }

//     #[func]
//     fn update_game_entities(&mut self, entities_data: Dictionary) {
//         let mut entities_to_keep = Dictionary::new();

//         for (_key, entity_variant) in entities_data.iter_shared() {
//             if let Ok(entity_data) = entity_variant.try_to::<Dictionary>() {
//                 let entity_id = entity_data.at("entity_id").to_string();
//                 if !self.entities.contains_key(entity_id.clone()) {
//                     if let Some(entity_scene) = &self.entity_scene {
//                         if let Some(instance) = entity_scene.instantiate() {
//                             if let Ok(mut sprite) = instance.try_cast::<Sprite2D>() {
//                                 sprite.set("entity_id", &entity_id.to_variant());
//                                 self.base_mut().add_child(&sprite.clone().upcast::<Node>());
//                                 self.entities.set(entity_id.clone(), sprite.to_variant());
//                                 entities_to_keep.set(entity_id.clone(), sprite.to_variant());

//                                 godot_print!("Entity created: {}", entity_id);
//                             }
//                         }
//                     }
//                 } else {
//                     if let Some(entity_node) = self.entities.get(entity_id.clone()) {
//                         entities_to_keep.set(entity_id.clone(), entity_node);
//                     }
//                 }
//             }
//         }

//         for (key, entity_node_variant) in self.entities.iter_shared() {
//             let entity_id = key.to_string();
//             if !entities_to_keep.contains_key(entity_id.clone()) {
//                 if let Ok(mut entity_node) = entity_node_variant.try_to::<Gd<Node>>() {
//                     entity_node.queue_free();
//                     godot_print!("Removed entity: {}", entity_id);
//                 }
//             }
//         }
//     }
// }

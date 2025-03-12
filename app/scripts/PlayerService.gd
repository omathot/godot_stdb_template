class_name PlayerService
extends Node

signal player_joined(player_id, player_name)
signal player_left(player_id)
signal player_renamed(player_id, old_name, new_name) 
signal operation_failed(operation, reason)

var player_module

func _init(db_manager):
	# Get the PlayerModule reference from DbManager
	player_module = db_manager.get_player_module()
	
	# Connect to module signals if available
	if player_module.has_signal("player_joined"):
		player_module.player_joined.connect(_on_player_joined)
	
	if player_module.has_signal("player_left"):
		player_module.player_left.connect(_on_player_left)
	
	if player_module.has_signal("player_renamed"):
		player_module.player_renamed.connect(_on_player_renamed)

# Public API - these methods will be called by game objects

func set_my_name(new_name: String) -> bool:
	if not player_module.get_connection_status():
		operation_failed.emit("set_name", "Not connected to database")
		return false
		
	return player_module.set_player_name(new_name)

func get_my_player() -> Dictionary:
	return player_module.get_my_player()

func get_all_players() -> Array:
	var players = []
	var all_players = player_module.get_all_players()
	
	for i in range(all_players.size()):
		players.append(all_players[i])
	
	return players

func get_player_by_id(player_id: int) -> Dictionary:
	var players = get_all_players()
	for player in players:
		if player.player_id == player_id:
			return player
	return {}

# Signal handlers
func _on_player_joined(player_id, player_name):
	player_joined.emit(player_id, player_name)

func _on_player_left(player_id):
	player_left.emit(player_id)
	
func _on_player_renamed(player_id, new_name):
	var old_name = ""
	var player = get_player_by_id(player_id)
	if not player.is_empty():
		old_name = player.get("name", "")
	
	player_renamed.emit(player_id, old_name, new_name)

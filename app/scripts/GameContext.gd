class_name GameContext
extends Node

# Services
var player_service: PlayerService
var entity_service: EntityService

# Core systems
var db_manager: DbManager
var game_manager: GameManager

signal connection_state_changed(connected)

func _init(db_manager_ref, game_manager_ref):
	# Store references to core systems
	db_manager = db_manager_ref
	game_manager = game_manager_ref
	
	# Connect to db_manager signals
	if db_manager.has_signal("connection_state_changed"):
		db_manager.connection_state_changed.connect(_on_connection_state_changed)
	
	# Initialize services
	player_service = PlayerService.new(db_manager)
	entity_service = EntityService.new(db_manager)
	
	# Add services as children for proper lifecycle management
	add_child(player_service)
	add_child(entity_service)

# Connection management
func is_connected_db() -> bool:
	return db_manager.is_connected()
	
func connect_to_game() -> bool:
	return db_manager.connect_to_db()
	
func disconnect_from_game() -> void:
	db_manager.disconnect_from_db()

# Signal handlers
func _on_connection_state_changed(connected):
	connection_state_changed.emit(connected)

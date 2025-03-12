extends Node

# Core systems (auto-loaded from scene)
@onready var db_manager = $DbManager
@onready var game_manager = $GameManager

# The game context provides centralized access to all services
var context: GameContext

func _ready():
	print("Game starting...")
	
	# Create our game context with access to core systems
	context = GameContext.new(db_manager, game_manager)
	add_child(context)
	
	# Connect to connection state changes
	context.connection_state_changed.connect(_on_connection_state_changed)
	
	# Make sure game_manager has access to context
	if game_manager.has_method("set_context"):
		game_manager.set_context(context)
	
	# Connect to the database
	if context.connect_to_game():
		print("Connected to SpaceTimeDB!")
	else:
		print("Failed to connect to SpaceTimeDB!")
		# You could show a connection error UI here

# Example method to demonstrate player name change
func _input(event):
	if event.is_action_pressed("ui_accept"):
		_change_player_name()

func _change_player_name():
	var new_name = "Player" + str(randi() % 1000)
	print("Trying to change name to: " + new_name)
	
	if context.player_service.set_my_name(new_name):
		print("Name changed successfully!")
	else:
		print("Failed to change name")

# Utility method to provide game objects with context
func setup_game_object(object: Node):
	if object.has_method("set_context"):
		object.set_context(context)

# Connection state handler
func _on_connection_state_changed(connected):
	if connected:
		print("Connected to database!")
		_after_connected()
	else:
		print("Disconnected from database!")
		# Handle reconnection or show error UI

# Flow after successful connection
func _after_connected():
	# Example: join game with random player name
	var player_name = "Player" + str(randi() % 1000)
	if context.player_service.set_my_name(player_name):
		print("Set player name to: " + player_name)
	else:
		print("Failed to set player name")
	
	# Example: print all players
	_print_all_players()

func _print_all_players():
	print("Current players:")
	var players = context.player_service.get_all_players()
	for player in players:
		print("- Player ", player.player_id, ": ", player.get("name", "unnamed"))

func _exit_tree():
	# Clean disconnection
	context.disconnect_from_game()

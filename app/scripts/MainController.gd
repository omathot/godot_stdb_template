extends Node

# Core systems (auto-loaded from scene)
@onready var db_manager = $DbManager
@onready var player_module = $PlayerModule

func _ready():
	print("Game starting...")
	#db_manager.connect_modules();
	if db_manager.connect_to_db():
		print("Connected to SpaceTimeDB!")
	else:
		print("Failed to connect to SpaceTimeDB!")

# Example method to demonstrate player name change
func _input(event):
	if event.is_action_pressed("ui_accept"):
		_change_player_name()

func _change_player_name():
	var new_name = "Player" + str(randi() % 1000)
	print("Trying to change name to: " + new_name)
	
	player_module.set_player_name(new_name);

func _exit_tree():
	# Clean disconnection
	get_tree().quit();

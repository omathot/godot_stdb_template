extends DbManager

# Main game scene that connects everything together

# References to our Rust components
var game_manager

func _ready():
	print("Game starting...")
	
	game_manager = GameManager.new()
	add_child(game_manager)
	
	if self.connect_to_db():
		print("Connected to SpaceTimeDB!")
	else:
		print("Failed to connect to SpaceTimeDB!")


func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_right"):
		var res = self.call_reducer("set_player_name", ["John"])
		if !res:
			printerr("Failed to set player name")
		else:
			print("Changed name")

func _process(_delta):
	pass

func _exit_tree():
	self.disconnect_from_db()
	

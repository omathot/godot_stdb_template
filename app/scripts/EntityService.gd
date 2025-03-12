class_name EntityService
extends Node

signal entity_created(entity_id, entity_data)
signal entity_updated(entity_id, entity_data)
signal entity_removed(entity_id)

var entity_module

func _init(db_manager):
	# Get the EntityModule reference from DbManager
	entity_module = db_manager.get_entity_module()
	
	# Connect to module signals if available
	if entity_module.has_signal("entity_created"):
		entity_module.entity_created.connect(_on_entity_created)
	
	if entity_module.has_signal("entity_updated"):
		entity_module.entity_updated.connect(_on_entity_updated)
	
	if entity_module.has_signal("entity_removed"):
		entity_module.entity_removed.connect(_on_entity_removed)

# Public API - these methods will be called by game objects

func get_all_entities() -> Array:
	var entities = []
	var all_entities = entity_module.get_all_entities()
	
	for i in range(all_entities.size()):
		entities.append(all_entities[i])
	
	return entities

func get_entity_by_id(entity_id: int) -> Dictionary:
	return entity_module.get_entity_by_id(entity_id)

# Signal handlers
func _on_entity_created(entity_id):
	var entity_data = get_entity_by_id(entity_id)
	entity_created.emit(entity_id, entity_data)

func _on_entity_updated(entity_id):
	var entity_data = get_entity_by_id(entity_id)
	entity_updated.emit(entity_id, entity_data)
	
func _on_entity_removed(entity_id):
	entity_removed.emit(entity_id)

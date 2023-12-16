extends PanelContainer


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func launch_gem_builder():
	var gem_builder_scene: PackedScene = preload("res://content/src/menus/gem_builder/gem_builder.tscn")
	var gem_builder = gem_builder_scene.instantiate()
	
	var window = Window.new()
	window.transient = true
	window.always_on_top = true
	window.initial_position = Window.WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN
	window.add_child(gem_builder)
	window.connect("close_requested", self._on_gem_builder_closed.bind(window))
	window.size = Vector2(800, 600)

	add_child(window)

func _on_gem_builder_closed(window: Window):
	remove_child(window)
	pass

func _on_create_pressed():
	launch_gem_builder()
	pass # Replace with function body.

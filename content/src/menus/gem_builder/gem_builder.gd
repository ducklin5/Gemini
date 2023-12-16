@tool
extends GemBuilderUI 

func _init():
	print("GDscript init")
	# Create a new Sprite node
	var label = Label.new()
	# Add the new sprite as a child of this Node2D
	add_child(label)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _ready():
	print("GDscript ready")


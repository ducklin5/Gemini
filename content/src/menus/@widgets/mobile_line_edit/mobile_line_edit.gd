class_name MobileLineEdit
extends LineEdit

#var screen_height = ProjectSettings.get_setting("display/window/size/height")
#var actual_resolution = OS.get_window_size()
#var set_position = false
#var global_position

#func reposition():
#	var target_y
#	if has_focus():
#		var virtualKeyboardHeight = OS.get_virtual_keyboard_height() * screen_height / actual_resolution.y
#		target_y = min(global_position.y, screen_height - virtualKeyboardHeight - get_size().y)
#	else:
#		target_y = global_position.y
#	set_global_position(Vector2(global_position.x, target_y))
#
#func _process(delta):
#	if set_position:
#		global_position = get_global_position()
#		set_position = false
#	reposition()

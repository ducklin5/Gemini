extends PanelContainer
class_name SubNav

signal tab_focused

func _ready():
	focus_entered.connect(_on_focus_entered.bind(self))
	get_child(0).tab_focused.connect(emit_tab_focused.bind(self))

func emit_tab_focused():
	emit_signal("tab_focused")

func _on_focus_entered():
	get_child(0).grab_focus()

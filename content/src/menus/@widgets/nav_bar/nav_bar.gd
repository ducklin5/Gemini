extends BoxContainer

@export_range(3, 64) var current: int = 3

@export var bar_size_ratio: float = 0.1: set = set_bar_size

enum Style {Fit = 0, Scroll = 1}
@export var BarStyle: Style = Style.Scroll
@export var ButtonScene: PackedScene

@export var enable_divider = false
@export var divider_texture: Texture

@export var dockable = false
@export_flags("right", "top", "left", "bottom") var dock_direction = 0
@export var docked_margin = -64
@export var opened_margin = 16
@export var is_open = true

@onready var tabs:Control = $Margin/Scroll/Tabs

var buttons: Dictionary = {}

var tween: Tween;

signal tabFocused

func _ready():
	$Margin.show()
	
	set_bar_size(bar_size_ratio)
	
	for idx in range(2, get_child_count()):
		var control :Control = get_child(idx)
		addTab(control)
	buttons[current].button_pressed = true
	
	if dockable:
		toggle_open(is_open)
		
	if enable_divider:
		$Divider.texture = divider_texture
		$Divider.show()

func set_bar_size( ratio ):
	bar_size_ratio = ratio
	$Margin.size_flags_stretch_ratio = bar_size_ratio

func addTab(target: Control):
	if ! (target in get_children()):
		add_child(target)
	target.size_flags_horizontal = SIZE_EXPAND_FILL
	target.size_flags_vertical = SIZE_EXPAND_FILL
	
	var idx = target.get_index()
	if idx == current: target.show()
	else: target.hide()
	
	var button: Button = ButtonScene.instantiate() if ButtonScene else Button.new()
	button.name = target.name
	button.text = target.name
	
	buttons[idx] = button
	
	if BarStyle == Style.Fit:
		button.size_flags_horizontal = SIZE_EXPAND_FILL
		button.size_flags_vertical = SIZE_EXPAND_FILL
	
	if target is SubNav:
		target.tab_focused.connect(toggle_open.bind(false))
	button.focus_entered.connect(set_current.bind(idx))
	button.pressed.connect(focus_target.bind(idx))
	button.focus_entered.connect(emit_tab_focused)
	
	if target.is_in_group("hideTab"):
		button.hide()
	else: 
		for tab in tabs.get_children():
			tab.focus_next = ""
	tabs.add_child(button)
	var firstTab = tabs.get_child(0)
	button.focus_next = firstTab.get_path()

func set_current(targetId):
	if targetId == current: return
	buttons[current].button_pressed = false
	buttons[targetId].button_pressed = true
	get_child(current).hide()
	get_child(targetId).show()
	current = targetId

func focus_target(targetId):
	var target :Control = get_child(targetId)
	if target.focus_mode:
		target.grab_focus()

func toggle_open(should_open):
	var prop = "custom_constants/margin_"+dock_direction
	var targetMargin;
	if should_open:
		targetMargin = opened_margin
		$Margin/Back.mouse_filter = Control.MOUSE_FILTER_IGNORE
	else:
		targetMargin = docked_margin
		$Margin/Back.mouse_filter = Control.MOUSE_FILTER_STOP
	
	if should_open == is_open:
		$Margin.set(prop, targetMargin)
		return
		
	tween = create_tween()
	tween.tween_property(
		$Margin, prop, 
		targetMargin, 1,
	).set_trans(Tween.TRANS_LINEAR)
	
	for tab in tabs.get_children():
		tab.disabled = !should_open
		tab.focus_mode = FOCUS_ALL if should_open else FOCUS_NONE
	is_open = should_open

func emit_tab_focused():
	emit_signal("tabFocused")

func _on_Back_pressed():
	_on_Margin_focus_entered()

func _on_focus_entered():
	_on_Margin_focus_entered()

func _on_Margin_focus_entered():
	toggle_open(true)
	buttons[current].grab_focus()

extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	var i = InterfaceBuilder.new()
	var y = i.make()
#	y.hello_world()
	print(y)
#	var y = i.hello_world()
#	i.hello_world()

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

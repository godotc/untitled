extends Node


var character:CharacterBody3D

# Called when the node enters the scene tree for the first time.
func _ready():
	character= get_parent()
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	
	var move_vec = Vector2(character.velocity.x, character.velocity.z);
	$AnimationTree.set("parameters/conditions/idle",
		move_vec== Vector2.ZERO && character.is_on_floor());
	$AnimationTree.set("parameters/conditions/walk", 
		move_vec!= Vector2.ZERO && character.is_on_floor());
	$AnimationTree.set("parameters/conditions/jump",
		 ! character.is_on_floor() &&  Input.is_action_pressed("space"));

	pass

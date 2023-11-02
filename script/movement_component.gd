extends MovementComponent
#

#
## Called when the node enters the scene tree for the first time.
func _ready():
	print(get_character_root())
	pass # Replace with function body.
#
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta:float):
	#var move_vec = Vector2(character.velocity.x, character.velocity.z);
	#self.animation_tree.set("parameters/conditions/idle",
		#move_vec== Vector2.ZERO && character.is_on_floor());
	#self.animation_tree.set("parameters/conditions/walk", 
		#move_vec!= Vector2.ZERO && character.is_on_floor());
	#self.animation_tree.set("parameters/conditions/jump",
		 #! character.is_on_floor() &&  Input.is_action_pressed("space"));
	pass;


extends CharacterBody3D


const SPEED = 5.0
const JUMP_VELOCITY = 4.5

@export var look_at_weight=0.2

# Get the gravity from the project settings to be synced with RigidBody nodes.
var gravity = ProjectSettings.get("physics/3d/default_gravity")

var lookat_node3d
var last_lookat: Vector3

func _ready():
	lookat_node3d = get_tree().get_nodes_in_group("CameraController")[0].get_node("LookAt");
	
func _physics_process(delta):
	## Add the gravity.
	if not is_on_floor():
		velocity.y -= gravity * delta
#
	## Handle jump.
	if Input.is_action_just_pressed("space") and is_on_floor():
		velocity.y = JUMP_VELOCITY
#
	## Get the input direction and handle the movement/deceleration.
	## As good practice, you should replace UI actions with custom gameplay actions.
	var input_dir = Input.get_vector("a", "d", "w", "s")
	#
	#var direction = (transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
	#if direction:
		#var lerped_lookat = lerp(
			#last_lookat, 
			#Vector3(lookat_node3d.global_position.x,
			 	##lerp(global_position.y, lookat_node3d.global_position.y , 0.7),
				#global_position.y,
			 	#lookat_node3d.global_position.z), 
			#look_at_weight);
		#look_at(lerped_lookat);
		#last_lookat = lerped_lookat;
		#
		#velocity.x = direction.x * SPEED
		#velocity.z = direction.z * SPEED
	#else:
		#velocity.x = move_toward(velocity.x, 0, SPEED)
		#velocity.z = move_toward(velocity.z, 0, SPEED)


	$AnimationTree.set("parameters/conditions/idle", input_dir== Vector2.ZERO && is_on_floor());
	$AnimationTree.set("parameters/conditions/walk", input_dir!= Vector2.ZERO&& is_on_floor());
	$AnimationTree.set("parameters/conditions/jump", !is_on_floor() &&  Input.is_action_pressed("space"));
	$AnimationTree.set("parameters/conditions/floating", !is_on_floor());
	#move_and_sttlide()
	pass

func  _input(event):
	
	pass

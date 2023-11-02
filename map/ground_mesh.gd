extends MeshInstance3D

@export
var height_scale:float =0.5;
# Called when the node enters the scene tree for the first time

func _init():
	#var shader = mesh.surface_get_material(0);
	var material = get_active_material(0)
	if material is ShaderMaterial:
		material.set_shader_parameter("height_scale", height_scale)
		pass
	pass
	
	
func _ready():
	pass
	


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

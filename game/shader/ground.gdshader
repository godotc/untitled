shader_type spatial;

varying vec2 tex_position;

uniform sampler2D noise;
uniform sampler2D normalmap;
uniform float height_scale = 0.5;

uniform sampler2D ground;
uniform float ground_tiling = 1.f;

void vertex() {
  tex_position = VERTEX.xz/2.0 +0.5;
  float height = texture(noise, tex_position).x;
  VERTEX.y += height* height_scale;

}

void fragment() {
	ROUGHNESS = 3.0;
	METALLIC = 0.3;
  NORMAL_MAP = texture(normalmap, tex_position).xyz;
	ALBEDO = texture(ground, tex_position * ground_tiling).xyz;
}



//void light() {
	//// Called for every pixel for every light affecting the material.
//}

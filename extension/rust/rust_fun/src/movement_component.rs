use godot::engine::{AnimationTree, CharacterBody3D, Input, InputEvent, Node};
use godot::prelude::utilities::move_toward;
use godot::prelude::*;

#[doc = "The movement component"]
#[derive(GodotClass)]
#[class(base=Node)]
struct MovementComponent {
    #[base]
    node: Base<Node>,

    #[export]
    character_root: Option<Gd<CharacterBody3D>>,

    #[export]
    animation_tree: Option<Gd<AnimationTree>>,

    last_lookat: Vector3,

    #[export]
    lookat_smoth_weight: real,

    #[export]
    speed: real,

    #[export]
    jump_velocity: real,
}

#[godot_api]
impl MovementComponent {
    #[func]
    fn hello(&self) {
        godot_print!("hello world");
    }
}

#[godot_api]
impl NodeVirtual for MovementComponent {
    fn init(node: Base<Node>) -> Self {
        godot_print!("The MovementComponent in rust init!");

        let mut component = Self {
            node,
            character_root: None,
            animation_tree: None,
            last_lookat: Vector3::ZERO,
            lookat_smoth_weight: 0.5,
            jump_velocity: 4.5,
            speed: 5.0,
        };

        if component.character_root.is_none() {
            component.character_root = component
                .node
                .get_parent()
                .and_then(|x| x.try_cast::<CharacterBody3D>());
        }

        if component.animation_tree.is_none() {
            component.animation_tree = component.character_root.as_ref().and_then(|x| {
                x.find_child("AnimationTree".into())
                    .and_then(|y| y.try_cast::<AnimationTree>())
            });
        }

        component
    }

    fn physics_process(&mut self, delta: f64) {
        godot_print!("Movemen componet tick {}", delta);
        if self.character_root.is_none() {
            return;
        }
        let character = self.character_root.as_deref_mut().unwrap();

        let input = Input::singleton();
        let input_vec = input.get_vector("a".into(), "d".into(), "s".into(), "w".into());

        let dir = character.get_transform().basis
            * Vector3 {
                x: input_vec.x,
                y: 0.0,
                z: input_vec.y,
            }
            .normalized();

        if dir != Vector3::ZERO {
            let forward = -character.get_global_transform().basis.rows[2];

            let lerped_lookat = Vector3::lerp(
                self.last_lookat,
                Vector3 {
                    x: forward.x,
                    y: character.get_global_position().y,
                    z: forward.z,
                },
                self.lookat_smoth_weight,
            );

            character.look_at(lerped_lookat);

            self.last_lookat = lerped_lookat;

            character.set_velocity(Vector3 {
                x: dir.x * self.speed,
                y: character.get_velocity().y,
                z: dir.z * self.speed,
            });
        } else {
            character.set_velocity(Vector3 {
                x: move_toward(character.get_velocity().x.into(), 0.0, self.speed.into()) as f32,
                y: character.get_velocity().y,
                z: move_toward(character.get_velocity().z.into(), 0.0, self.speed.into()) as f32,
            });
        }

        character.move_and_slide();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("e".into()) {
            self.hello();
        }
    }
}

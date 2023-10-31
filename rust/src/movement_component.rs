use godot::engine::{AnimationTree, CharacterBody2D, CharacterBody3D, Input, InputEvent, Node};
use godot::prelude::utilities::move_toward;
use godot::prelude::*;

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

        component.character_root = component
            .node
            .get_parent()
            .and_then(|x| x.try_cast::<CharacterBody3D>());

        // Specify by editor now....
        // if component.parent.is_some() {
        //     godot_print!(
        //         "Found the parent {}",
        //         component.parent.as_ref().unwrap().to_string()
        //     );
        // } else {
        //     println!("Not found")
        // }
        // let machine = component.node.find_child("AnimationTree".into());
        // if machine.is_some() {
        //     component.animation_tree = machine.unwrap().try_cast::<AnimationTree>();
        // }

        component
    }

    fn process(&mut self, delta: f64) {
        godot_print!("abacacdac");
        if self.character_root.is_none() {
            return;
        }
        let mut character = self.character_root.as_deref_mut().unwrap();

        let input = Input::singleton();
        let input_vec = input.get_vector("a".into(), "d".into(), "s".into(), "w".into());

        let dir = Vector3::ONE
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

    fn input(&mut self, event: Gd<InputEvent>) {}
}

use godot::builtin::GodotString;
use godot::engine::{CharacterBody3D, InputEvent, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct MovementComponent {
    speed: f32,
    jump: bool,

    parent: Option<Gd<CharacterBody3D>>,

    #[base]
    node: Base<Node>,
}

#[godot_api]
impl MovementComponent {
    #[func]
    fn hello_world(&self, who: GodotString) {
        self.hello_internal(who);
    }
    #[func]
    fn bello_world(&self, who: GodotString) {}

    fn hello_internal(&self, who: GodotString) {
        static I: i32 = 1;
        let x = 2;
        if I != x {
            godot_print!("hello world! {} {}", I, who.to_string());
        }
    }
}

#[godot_api]
impl NodeVirtual for MovementComponent {
    fn init(node: Base<Node>) -> Self {
        godot_print!("The MovementComponent in rust init!");
        let Component = Self {
            speed: 0.0,
            jump: false,
            node,
            parent: None,
        };
        // Component.node.get_parent().unwrap();
        Component
    }

    fn physics_process(&mut self, delta: f64) {
        self.speed = self.speed.sin() * 200.0 * delta as f32 + 1.5;
        // godot_print!("The new speed {}", self.speed);
    }

    fn input(&mut self, event: Gd<InputEvent>) {}
}

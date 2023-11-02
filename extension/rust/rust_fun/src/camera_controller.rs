use godot::engine::{InputEvent, InputEventMouseMotion};
use godot::prelude::*;

#[doc = "The movement component"]
#[derive(GodotClass)]
#[class(base=Node3D)]
struct CameraController {
    #[base]
    node: Base<Node3D>,

    #[export]
    sensitivity: real,
}

#[godot_api]
impl CameraController {
    #[func]
    fn hello(&self) {
        godot_print!("hello world");
    }
}

#[godot_api]
impl Node3DVirtual for CameraController {
    fn init(node: Base<Node3D>) -> Self {
        let component = Self {
            node,
            sensitivity: 5.0,
        };

        component
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let ev = event.try_cast::<InputEventMouseMotion>();
        if ev.is_some() {
            let mut roation = self.node.get_rotation();
            let relative = ev.unwrap().get_relative();

            let temp_rotaion_x = roation.x - relative.y / 1000.0 * self.sensitivity;

            roation.y -= relative.x / 1000.0 * self.sensitivity;
            roation.x = temp_rotaion_x.clamp(-0.6, -0.1);

            self.node.set_rotation(roation);
        }
    }

}


use godot::engine::{Camera3D, InputEvent, InputEventMouseMotion, Node3D};
use godot::prelude::*;

#[doc = "The movement component"]
#[derive(GodotClass)]
#[class(base=Node3D)]
struct CameraController {
    #[base]
    pivot: Base<Node3D>,

    #[export]
    sensitivity: real,

    #[export]
    caemra: Option<Gd<Camera3D>>,
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
    fn init(pivot: Base<Node3D>) -> Self {
        let mut component = Self {
            pivot,
            sensitivity: 5.0,
            caemra: Some(Camera3D::new_alloc()),
        };

        component
            .pivot
            .add_child(component.get_caemra().unwrap().get_node::<Node>());

        component
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let ev = event.try_cast::<InputEventMouseMotion>();
        if ev.is_some() {
            let mut roation = self.pivot.get_rotation();
            let relative = ev.unwrap().get_relative();

            let temp_rotaion_x = roation.x - relative.y / 1000.0 * self.sensitivity;

            roation.y -= relative.x / 1000.0 * self.sensitivity;
            roation.x = temp_rotaion_x.clamp(-0.6, -0.1);

            self.pivot.set_rotation(roation);
        }
    }
}

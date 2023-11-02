use godot::prelude::*;


struct RustFanExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustFanExtension {}

mod movement_component;
mod camera_controller;
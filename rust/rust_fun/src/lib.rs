use godot::prelude::*;

mod movement_component;

struct RustFanExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustFanExtension {}

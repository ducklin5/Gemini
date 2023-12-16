use godot::engine::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object, tool)]
pub struct GemFragment {}

#[godot_api]
pub impl IObject for GemFragment {
    fn init(mut _base: Base<Object>) -> Self {
        Self {}
    }
}

#[godot_api]
pub impl GemFragment {
    #[func]
    pub fn get_name(&self) -> GString {
        "BaseFragment".into()
    }
}

struct OutputFragment {}

struct ScriptFragment {}

struct InputFragment {}

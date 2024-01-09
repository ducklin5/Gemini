use godot::engine::*;
use godot::prelude::*;
use crate::core::gem_fragment::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GemFragmentGenerator {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for GemFragmentGenerator {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
        }
    }
}
#[godot_api]
impl GemFragmentGenerator {
    pub fn generate(&mut self) -> Gd<GemFragment> {
        GemFragment::new_gd()
    }
}

use std::collections::HashMap;

use godot::prelude::*;

use super::gem_fragment::GemFragment;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct GemFragmentLink {
    #[base]
    base: Base<Resource>,

    #[export]
    pub src_id: GString,

    #[export]
    pub snk_id: GString,

    #[export]
    pub key: GString,
}

#[godot_api]
pub impl IResource for GemFragmentLink {
    fn init(base: Base<Self::Base>) -> Self {
        

        Self {
            base,
            src_id: GString::new(),
            snk_id: GString::new(),
            key: GString::new(),
        }
    }
}

#[godot_api]
impl GemFragmentLink {
    #[func]
    fn rename(&self) {
        self.to_gd().set_name(format!("{} -> {} ({})", self.src_id, self.snk_id, self.key).into());
    }
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Gem {
    #[export]
    pub input_fragments: Array<Gd<GemFragment>>,

    #[export]
    pub driver_fragments: Array<Gd<GemFragment>>,

    #[export]
    pub output_fragments: Array<Gd<GemFragment>>,

    #[export]
    pub input_driver_links: Array<Gd<GemFragmentLink>>,

    #[export]
    pub driver_output_links: Array<Gd<GemFragmentLink>>,
}

#[godot_api]
pub impl IResource for Gem {
    fn init(_base: Base<Resource>) -> Self {
        Self {
            input_fragments: Array::new(),
            driver_fragments: Array::new(),
            output_fragments: Array::new(),
            input_driver_links: Array::new(),
            driver_output_links: Array::new(),
        }
    }
}

impl Gem {
    pub fn get_fragments(&self) -> HashMap<(usize, usize), Gd<GemFragment>> {
        let fragment_arrays = vec![
            self.input_fragments.clone(),
            self.driver_fragments.clone(),
            self.output_fragments.clone(),
        ];
        fragment_arrays
            .iter()
            .enumerate()
            .flat_map(|(arr_idx, array)| {
                array
                    .iter_shared()
                    .enumerate()
                    .map(move |(frag_idx, frag)| ((arr_idx.clone(), frag_idx), frag))
            })
            .collect()
    }

    pub fn process(&self, node: Gd<Node>, delta: f64) {
        self.get_fragments().iter().for_each(|(_id, frag_ref)| {
            let mut tmp_frag_ref = frag_ref.clone();
            let mut frag = tmp_frag_ref.bind_mut();

            let inputs: HashMap<String, String> = HashMap::new();

            //frag.process(node.clone(), inputs, delta)
        })
    }
}

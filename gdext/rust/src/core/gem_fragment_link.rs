use std::collections::HashMap;

use crate::emit_signal;
use godot::prelude::*;

use super::gem_fragment::GemFragment;

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct GemFragmentLink {
    #[base]
    base: Base<Resource>,

    #[export]
    #[var(set = set_src_id, get)]
    pub src_id: GString,

    #[export]
    #[var(set = set_snk_id, get)]
    pub snk_id: GString,

    #[export]
    #[var(set = set_key, get)]
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
    #[signal]
    fn prop_changed(&self);
    
    #[func]
    pub fn rs_init(&mut self) {
        self.rename();
        let mut this = self.base_mut();
        let this_rename = this.callable("rename");
        this.connect("prop_changed".into(), this_rename);
    }

    #[func]
    fn set_src_id(&mut self, id: GString) {
        self.src_id = id;
        let mut this = self.base_mut();
        this.emit_signal("prop_changed".into(), &[]);
    }

    #[func]
    fn set_snk_id(&mut self, id: GString) {
        self.snk_id = id;
        let mut this = self.base_mut();
        this.emit_signal("prop_changed".into(), &[]);
    }

    #[func]
    fn set_key(&mut self, key: GString) {
        self.key = key;
        let mut this = self.base_mut();
        this.emit_signal("prop_changed".into(), &[]);
    }

    #[func]
    fn rename(&mut self) {
        let new_name = format!("{} -> {} ({})", self.src_id, self.snk_id, self.key).into();
        self.base_mut().set_name(new_name);
    }
}

pub trait Searchable<T: GodotClass> {
    fn find_id(&self, id: &GString) -> Option<Gd<T>>;
}

impl Searchable<GemFragment> for Array<Option<Gd<GemFragment>>> {
    fn find_id(&self, id: &GString) -> Option<Gd<GemFragment>> {
        self.iter_shared()
            .find(|frag_opt| {
                frag_opt
                    .as_ref()
                    .map(|frag_ref| frag_ref.bind().id() == *id)
                    .unwrap_or(false)
            })
            .flatten()
    }
}

type LinkValidationResult = Result<(), Array<Option<Gd<GemFragmentLink>>>>;

pub fn verify_links(
    links: &Array<Option<Gd<GemFragmentLink>>>,
    src: &Array<Option<Gd<GemFragment>>>,
    snk: &Array<Option<Gd<GemFragment>>>,
) -> LinkValidationResult {
    let result = links
        .iter_shared()
        .filter(|link_opt| {
            if let Some(link_ref) = link_opt {
                let link = link_ref.bind();
                let src_frag = src.find_id(&link.src_id);
                let snk_frag = snk.find_id(&link.snk_id);
                return match (src_frag, snk_frag) {
                    (Some(_), Some(_)) => false,
                    _ => true,
                };
            }
            return true;
        })
        .collect::<Array<Option<Gd<GemFragmentLink>>>>();
    if result.len() > 0 {
        Err(result)
    } else {
        Ok(())
    }
}

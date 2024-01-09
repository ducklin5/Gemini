use super::gem_fragment_card::*;
use super::gem_fragment_generator::*;
use crate::core::gem_fragment::*;
use control::SizeFlags as SF;
use godot::engine::*;
use godot::prelude::*;

use crate::{add_new_child, config_node};

#[derive(GodotClass)]
#[class(base=VBoxContainer, tool)]
pub struct GemFragmentCardList {
    #[base]
    base: Base<VBoxContainer>,

    #[export]
    #[var(set = set_title, get = get_title)]
    title: GString,

    #[export]
    #[var(set = set_action_text, get = get_action_text)]
    action_text: GString,
}

#[godot_api]
impl IVBoxContainer for GemFragmentCardList {
    fn init(base: Base<VBoxContainer>) -> Self {
        let fragment =  GemFragment::new_gd();
        let mut base_gd = base.to_gd();
        config_node!(base_gd,
            [
                "size_flags_horizontal" =>  SF::SIZE_EXPAND_FILL
            ] {
                Label ("Title") [
                    "size_flags_horizontal" =>  SF::SIZE_SHRINK_CENTER
                ],
                ScrollContainer("Container") [
                    "size_flags_vertical" =>  SF::SIZE_EXPAND_FILL
                ] {
                    VBoxContainer("List") [
                        "size_flags_horizontal" =>  SF::SIZE_EXPAND_FILL
                    ]
                },
                Button ("Action")
            }
        );
        
        let this = Self {
            base,
            title: "Title".into(),
            action_text: "Add Item".into(),
        };

        //this.to_gd().print_tree();
        let mut action_btn = base_gd.get_node("Action".into()).unwrap();
        action_btn.connect(
            "pressed".into(),
            Callable::from_object_method(&base_gd, "on_action_clicked"),
        );

        this
    }
}

#[godot_api]
impl GemFragmentCardList {
    #[func]
    pub fn set_title(&mut self, value: GString) {
        self.base_mut().get_node("Title".into()).map_or_else(
            || godot_print!("Title Node is missing!"),
            |mut node| {
                config_node!(node, [ "text" => value.clone() ]);
            },
        );
        self.title = value;
    }

    #[func]
    pub fn get_title(&mut self) -> GString {
        self.title.clone()
    }

    #[func]
    pub fn set_action_text(&mut self, value: GString) {
        self.base_mut().get_node("Action".into()).map_or_else(
            || godot_print!("Action Node is missing!"),
            |mut node| {
                config_node!(node, [ "text" => value.clone() ]);
            },
        );
        self.action_text = value;
    }

    #[func]
    pub fn get_action_text(&mut self) -> GString {
        self.action_text.clone()
    }

    #[func]
    pub fn on_action_clicked(&mut self) {
        godot_print!("Action Clicked!");
        let opt_frag_gen = self
            .to_gd()
            .get_children()
            .iter_shared()
            .find_map(|e| e.try_cast::<GemFragmentGenerator>().ok());
        if let Some(mut frag_gen) = opt_frag_gen {
            godot_print!("Generator found!");
            let mut frag_gen_ref = frag_gen.bind_mut();
            let fragment =  GemFragment::new_gd();
            godot_print!("Fragment generated");
            //self.add_fragment(fragment);
        } else {
            godot_print!("Missing GemFragmentGenerator Child!");
        }
    }

    #[func]
    pub fn add_fragment(&mut self, fragment: Gd<GemFragment>) {
        //let name = fragment.bind().get_name();
        let name: GString = "GemFragmentCard".into();
        let mut list_node = self.to_gd().get_node("Container/List".into()).unwrap();
        add_new_child!(list_node, GemFragmentCard(name)[
            "fragment" => fragment
        ]);
    }
}

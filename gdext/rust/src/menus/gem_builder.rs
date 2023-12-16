use godot::engine::*;
use godot::obj::UserClass;
use godot::prelude::*;
use control::SizeFlags as SF;
use control::LayoutPreset as LP;
use crate::{config_node, add_new_child};
use super::gem_fragment::*;


#[derive(GodotClass)]
#[class(base=Node)]
struct GemFragmentGenerator;

#[godot_api]
impl INode for GemFragmentGenerator {
    fn init(mut _base: Base<Node>) -> Self {
        Self {}
    }
}

#[godot_api]
impl GemFragmentGenerator {
    pub fn new_alloc() -> Gd<Self> {
        Self::alloc_gd()
    }
    
    #[func]
    pub fn generate(&mut self) -> Gd<GemFragment> {
       GemFragment::alloc_gd()
    }
}

#[derive(GodotClass)]
#[class(base=PanelContainer, tool)]
struct GemFragmentCard {
    #[base]
    base: Base<PanelContainer>,

    #[export]
    #[var(set = set_fragment, get = get_fragment)]
    fragment: Option<Gd<GemFragment>>,
    
    #[export]
    title: GString,
    
    #[export]
    action_text: GString,
    
}

#[godot_api]
impl IPanelContainer for GemFragmentCard {
    fn init(mut base: Base<PanelContainer>) -> Self {
        config_node!(base, [
                "size" => Vector2::new(0.0, 96.0),
                "anchors_preset" => LP::PRESET_TOP_WIDE
            ] {
                Panel("BG") ,
                VBoxContainer("Layout") {
                    Label("Label"),
                    HBoxContainer("Body")[
                        "size_flags_vertical" =>  SF::SIZE_EXPAND_FILL
                    ]
                    {
                        VBoxContainer("Info") [
                            "size_flags_vertical" =>  SF::SIZE_EXPAND_FILL
                        ],
                        Button("Action") [
                            "size" => Vector2::new(64.0, 0.0)
                        ]
                    }
                }
            }
        );
        GemFragmentCard {
            base,
            fragment: None,
            title: "Title".into(),
            action_text: "Edit".into(),
        }
    }
}

#[godot_api]
impl GemFragmentCard {
    fn new_alloc() -> Gd<GemFragmentCard> {
        Self::alloc_gd()
    }

    #[func]
    fn set_fragment(&mut self, fragment: Gd<GemFragment>) {
        self.fragment = Some(fragment.clone());
        self.title = fragment.bind().get_name();
    }

    #[func]
    fn get_fragment(&self) -> Option<Gd<GemFragment>> {
        self.fragment.clone()
    }
}

#[derive(GodotClass)]
#[class(base=VBoxContainer, tool)]
struct GemFragmentCardList {
    #[base]
    base: Base<VBoxContainer>,

    #[export]
    #[var(set = set_title, get = get_title)]
    title: GString,

    #[export]
    #[var(set = set_action_text, get = get_action_text)]
    action_text: GString
}

#[godot_api]
impl IVBoxContainer for GemFragmentCardList {
    fn init(mut base: Base<VBoxContainer>) -> Self {
        config_node!(base,
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

        let mut action_btn = base.get_node("Action".into()).unwrap();
        action_btn.connect(
            "pressed".into(),
            Callable::from_object_method(&base, "on_action_clicked"),
        );

        Self {
            base,
            title: "Title".into(),
            action_text: "Add Item".into(),
        }
    }
}


#[godot_api]
impl GemFragmentCardList {
    pub fn new_alloc() -> Gd<Self> {
        Self::alloc_gd()
    }

    #[func]
    pub fn set_title(&mut self, value: GString) {
        self.base.get_node("Title".into()).map_or_else(
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
        self.base.get_node("Action".into()).map_or_else(
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
        let opt_frag_gen = self
            .base
            .get_children()
            .iter_shared()
            .find_map(|e| e.try_cast::<GemFragmentGenerator>().ok());
        if let Some(mut frag_gen) = opt_frag_gen {
            let mut frag_gen_ref = frag_gen.bind_mut();
            let fragment = frag_gen_ref.generate();
            self.add_fragment(fragment);
        } else {
            godot_print!("Missing GemFragmentGenerator Child!");
        }
    }

    #[func]
    pub fn add_fragment(&mut self, fragment: Gd<GemFragment>) {
        let name = fragment.bind().get_name();
        let mut list_node = self.base.get_node("Container/List".into()).unwrap();
        add_new_child!(list_node, GemFragmentCard(name)[
            "fragment" => fragment
        ]);
    }
}

#[derive(GodotClass)]
#[class(base=PanelContainer, tool)]
struct GemBuilderUI;

#[godot_api]
impl IPanelContainer for GemBuilderUI {
    fn init(mut base: Base<PanelContainer>) -> Self {

        config_node!(base, {
            Panel ("BG"),
            VBoxContainer ("Sections") {
                Label("Header") [
                    "text" => "Gem Builder!"
                ],
                HBoxContainer ("Content") [
                    "size_flags_vertical" =>  SF::SIZE_EXPAND_FILL
                    ] {
                    GemFragmentCardList ("Inputs") [
                        "title" => "Inputs",
                        "action_text" => "Add Input"
                        ] {
                        GemFragmentGenerator
                    },
                    GemFragmentCardList ("Scripts") [
                        "title" => "Scripts",
                        "action_text" => "Add Script"],
                    GemFragmentCardList ("Outputs") [
                        "title" => "Outputs",
                        "action_text" => "Add Output"],
                },
                PanelContainer ("Footer")
            }
        });

        Self {}
    }
}

#[godot_api]
impl GemBuilderUI {}

use godot::engine::*;
use godot::prelude::*;

use control::LayoutPreset as LP;
use control::SizeFlags as SF;

use crate::config_node;

use crate::core::gem_fragment::*;


#[derive(GodotClass)]
#[class(base=PanelContainer, tool)]
pub struct GemFragmentCard {
    #[base]
    base: Base<PanelContainer>,

    #[export]
    #[var(set = set_fragment, get = get_fragment)]
    fragment: Option<Gd<GemFragment>>,
}

#[godot_api]
impl IPanelContainer for GemFragmentCard {
    fn init(base: Base<PanelContainer>) -> Self {
        config_node!(base.to_gd(), [
                "custom_minimum_size" => Vector2::new(0.0, 96.0),
                "size_flags_horizontal" => SF::SIZE_EXPAND_FILL
            ] {
                Panel("BG") ,
                MarginContainer("Padding") [
                    "theme_override_constants/margin_left" => 8,
                    "theme_override_constants/margin_bottom" => 8,
                    "theme_override_constants/margin_top" => 8,
                    "theme_override_constants/margin_right" => 8
                ] {
                    VBoxContainer("Layout") {
                        Label("Label") [
                            "custom_minimum_size" => Vector2::new(32.0, 0.0),
                            "text" => "GemFragmentCard"
                        ],
                        HBoxContainer("Body")[
                            "size_flags_vertical" =>  SF::SIZE_EXPAND_FILL
                        ]
                        {
                            Button("Move") [
                                "size_flags_horizontal" =>  SF::SIZE_EXPAND_FILL,
                                "text" => "Move"
                            ],
                            Button("Edit") [
                                "size_flags_horizontal" =>  SF::SIZE_EXPAND_FILL,
                                "text" => "Edit"
                            ],
                            Button("Delete") [
                                "size_flags_horizontal" =>  SF::SIZE_EXPAND_FILL,
                                "text" => "Delete"
                            ]
                        }
                    }
                }
            }
        );
        base.to_gd().set_anchors_preset(LP::PRESET_TOP_WIDE);
        GemFragmentCard {
            base,
            fragment: None,
        }
    }
}

#[godot_api]
pub impl GemFragmentCard {
    #[func]
    fn set_fragment(&mut self, fragment: Gd<GemFragment>) {
        self.fragment = Some(fragment.clone());
        self.to_gd()
            .try_get_node_as::<Label>("Padding/Layout/Label")
            .map(|mut node| {
                //node.set_text(fragment.bind().get_name());
                node.set_text("test".into());
            });
    }

    #[func]
    fn get_fragment(&self) -> Option<Gd<GemFragment>> {
        self.fragment.clone()
    }
}

use godot::engine::*;
use godot::prelude::*;
use control::SizeFlags as SF;
use crate::config_node;
use crate::core::gem::Gem;
use crate::core::gem_fragment::GemFragment;
use super::gem_fragment_card_list::*;
use super::gem_fragment_generator::*;

#[derive(GodotClass)]
#[class(base=PanelContainer, tool)]
struct GemBuilderUI;

#[godot_api]
impl IPanelContainer for GemBuilderUI {
    fn init(base: Base<PanelContainer>) -> Self {
        let mut base_gd = base.to_gd();
        let gem = GemFragment::new_gd();
        config_node!(base_gd, {
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
                    GemFragmentCardList ("Drivers") [
                        "title" => "Drivers",
                        "action_text" => "Add Driver"] {
                        GemFragmentGenerator

                    },
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

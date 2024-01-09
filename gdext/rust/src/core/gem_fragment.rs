use std::collections::HashMap;

use godot::prelude::*;

use rand::Rng;
use std::iter;

fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

#[repr(i64)]
#[derive(Var, Export, Default, Debug)]
pub enum GemFragmentClass {
    #[default]
    None = 0,
    InputSonar = 1,
    InputLight = 2,
    InputHeat = 3,
    InputElectric = 4,
    InputEther = 5,
    DriverDirective = 6,
    DriverProcedural = 7,
    DriverNatural = 8,
    DriverCyclic = 9,
    OutputThruster = 10,
    OutputFlagela = 11,
    OutputShockWave = 12,
    OutputRotar = 13,
    OutputHeatWave = 14,
    OutputLightBeam = 15,
    OutputSonicWave = 16,
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct GemFragment {
    #[base]
    base: Base<Resource>,

    // class: GemFragmentClass,

    //nodes: Option<Array<Gd<Node>>>,
}

#[godot_api]
pub impl IResource for GemFragment {
    fn init(base: Base<Resource>) -> Self {
        //let id = generate(4);

        let mut this = Self {
            base,
            // class: GemFragmentClass::None,
            //nodes: None,
        };
        
        this.base();
        this
    }

    fn to_string(&self) -> GString {
        //format!("{:#?}", self.class).into()
        "test".into()
    }
}
/*
#[godot_api]
pub impl GemFragment {
    #[func]
    pub fn get_name(&self) -> GString {
   //     use GemFragmentClass as FC;
   //     match self.class {
   //         FC::InputSonar => "Sonar",
   //         FC::InputHeat => "Heat",
   //         FC::InputLight => "Light",
   //         FC::InputElectric => "Electric",
   //         FC::InputEther => "Ether",
   //         FC::DriverCyclic => "Cyclic",
   //         FC::DriverNatural => "Natural",
   //         FC::DriverDirective => "Directive",
   //         FC::DriverProcedural => "Procedural",
   //         FC::OutputRotar => "Rotar",
   //         FC::OutputFlagela => "Flagela",
   //         FC::OutputThruster => "Thruster",
   //         FC::OutputHeatWave => "Heat Wave",
   //         FC::OutputShockWave => "Shock Wave",
   //         FC::OutputLightBeam => "Light Beam",
   //         FC::OutputSonicWave => "Sonic Wave",
   //         FC::None => "Uninitialized Fragment",
   //     }
   //     .into()
    "Uninitialized Fragment".into()
    
    }

    fn build_nodes(&mut self) -> Array<Gd<Node>> {
        let mut nodes = Array::<Gd<Node>>::new();
        //match self.class {
        //    GemFragmentClass::InputSonar => {
        //        let mut input_sonar_area = Area2D::new_alloc();
        //        config_node!(input_sonar_area, ("InputSonarArea"));
        //        nodes.push(input_sonar_area.upcast());
        //    }
        //    _ => {}
        //}
        nodes
    }

    #[func]
    pub fn init_nodes(&mut self) -> Array<Gd<Node>> {
        let nodes = self.build_nodes();
        //self.nodes = Some(nodes.clone());
        nodes
    }

   pub fn process(&mut self, owner: Gd<Node>, inputs: HashMap<String, String>, _delta: f64) {
   //     match self.class {
   //         GemFragmentClass::InputSonar => {
   //             self.nodes.clone().map(|nodes| {
   //                 let input_sonar_area = nodes.get(0).cast::<Area2D>();
   //                 let areas: Array<Gd<Area2D>> = input_sonar_area.get_overlapping_areas();
   //                 let sonic_data: Vec<(Vector2, i32)> = areas
   //                     .iter_shared()
   //                     .filter(|area: &Gd<Area2D>| {
   //                         area.is_in_group("OutputSonicWave".into())
   //                             && !area.is_ancestor_of(owner.clone())
   //                     })
   //                     .map(|source_area: Gd<Area2D>| {
   //                         let dir = source_area.get_global_position()
   //                             - input_sonar_area.get_global_position();
   //                         let integrity = 0; // TODO: calculate integrity (goodness/badness) of a sound
   //                         (dir, integrity)
   //                     }).collect();
   //                 let msg = format!("{:#?}", sonic_data);
   //             });
   //         },
   //         GemFragmentClass::OutputThruster => {
   //             
   //         },
   //         GemFragmentClass::None => {}
   //         _ => {}
   //     }
   }
}
*/

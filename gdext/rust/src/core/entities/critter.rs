use std::collections::HashMap;
use std::iter::Iterator;

use godot::engine::*;
use godot::prelude::*;

use crate::core::gem::Gem;
use crate::core::gem_fragment::GemFragment;
use crate::{config_node, path2imgtex};

#[derive(GodotClass)]
#[class(base=CharacterBody2D, tool)]
pub struct Critter {
    #[base]
    base: Base<CharacterBody2D>,

    //#[export]
    gem: Option<Gd<Gem>>,

    #[export]
    health: i32,

    #[export]
    energy: i32,

    #[export]
    stability: i32,

    // nodes that need to be freed when new gem is set
    // indexed by fragment id
    gem_nodes: HashMap<(usize, usize), Array<Gd<Node>>>,

    age: f64,
}

#[godot_api]
pub impl ICharacterBody2D for Critter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let tex = path2imgtex!("res://assets/entities/critter.png");
        //let mat = load::<Material>("res://assets/entities/critter.material");
        config_node!(base.to_gd(),{
            Sprite2D("Body") [
                "texture" => tex,
                "use_parent_material" => true
            ],
        });

        Self {
            base,
            gem: None,
            health: 100,
            energy: 100,
            stability: 100,
            gem_nodes: HashMap::new(),
            age: 0.0,
        }
    }
    fn process(&mut self, delta: f64) {
        self.age += delta;
        
        let self_node: Gd<Node> = self.to_gd().upcast();

        self.gem.clone().map(|g| {
            g.bind().process(self_node, delta)
        });
    }
}

#[godot_api]
impl Critter {
    fn all_gem_nodes(&self) -> impl Iterator<Item = Gd<Node>> + '_ {
        self.gem_nodes
            .iter()
            .flat_map(|(_, nodes)| nodes.iter_shared())
    }

    #[func]
    fn use_gem(&mut self, opt_gem: Option<Gd<Gem>>) {
        self.all_gem_nodes().for_each(|mut node| node.queue_free());
        self.gem_nodes.clear();
        self.gem = opt_gem.clone();

        if let Some(gem_ref) = opt_gem {
            let gem = gem_ref.bind();
            gem.get_fragments()
                .iter()
                .for_each(|(frag_id, frag)| self.add_fragment(*frag_id, frag.clone()));
        }
    }

    fn add_fragment(&mut self, frag_id: (usize, usize), mut fragment: Gd<GemFragment>) {
        /*let nodes = fragment.bind_mut().init_nodes();
        nodes.iter_shared().for_each(|node| {
            self.to_gd().add_child(node.clone());
        });

        self.gem_nodes.insert(frag_id, nodes);*/
    }
}

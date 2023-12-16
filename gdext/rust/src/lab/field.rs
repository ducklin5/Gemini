

use crate::scene;
use godot::engine::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D, tool)]
struct Field2D {
    #[base]
    root: Base<Node2D>,
}

#[godot_api]
impl INode2D for Field2D {
    fn init(mut root: Base<Node2D>) -> Self {
        let mut layer1_gradient = Gradient::new();
        let colors = [Color::from_hsv(0.7, 0.8, 0.9)];
        layer1_gradient.set_colors(PackedColorArray::from_iter(colors));
        let mut layer1_tex = GradientTexture2D::new();
        layer1_tex.set_gradient(layer1_gradient);

        let layer2_tex = GradientTexture2D::new();

        scene!(root, {
            Sprite2D("Layer1K")[
                "texture" =>  layer1_tex
            ]{},
            Sprite2D("Layer2")[
                "texture" => layer2_tex
            ]{},
        });


        Self { root }
    }
}

#[godot_api]
impl Field2D {}

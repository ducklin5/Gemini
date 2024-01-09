mod core;
mod menus;
mod macros;


use godot::prelude::*;

struct GeminiExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GeminiExtension {
    
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}

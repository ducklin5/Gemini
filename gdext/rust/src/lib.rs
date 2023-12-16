mod lab;
mod menus;
mod macros;


use godot::prelude::*;

struct GeminiExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GeminiExtension {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}

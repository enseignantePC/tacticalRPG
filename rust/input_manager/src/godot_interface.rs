//! Godot Interface
//!
//! This module gives the type Interface,
//! which is used to do all the taking and converting
//! with godot.

use gdnative::prelude::*;

#[derive(NativeClass, Debug, Clone)]
#[inherit(Reference)]
pub struct InterfaceBuilder {}

use gdnative::prelude::*;

#[methods]
impl InterfaceBuilder {
    fn new(_owner: &Reference) -> Self {
        InterfaceBuilder {}
    }
    #[export]
    pub fn init(
        &self,
        _owner: &Reference,
        x: Dictionary,
    ) {
        let y = x.get("some");
        dbg!(&y);
        // let try_try: Option<_> = y.try_to_object::<Node>();
        godot_print!(
            "{:?}", y,
            // try_try
        );
    }

    #[export]
    /// Given the option submitted before returns
    /// an Interface or Failed=1
    pub fn try_build(
        &self,
        _owner: &Reference,
    ) -> Variant {
        // Variant::from_u64(GodotError::Failed {} as u64)
        Interface::new_instance().owned_to_variant()
    }
}

#[derive(NativeClass, Debug, Clone)]
#[inherit(Reference)]
pub struct Interface {}

#[methods]
impl Interface {
    fn new(_owner: &Reference) -> Self {
        Interface {}
    }

    #[export]
    pub fn hello_world(
        &self,
        _owner: &Reference,
    ) {
        godot_print!("Hey from subclass!");
        println!("Hey from rust");
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<InterfaceBuilder>();
    handle.add_class::<Interface>();
}
// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

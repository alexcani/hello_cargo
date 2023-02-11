 // Example of procedural macro: custom derive
 pub trait HelloMacroTrait {
    fn hello_macro_fn();
}

pub trait Teste {
    fn ha();
}
// Custom derive macros need to be defined in their own crate
// The convention is a crate named foo has a custom derive macro crate called foo_derive
// Check hello_cargo_derive crate


// Re-export the derive macro
pub use my_lib_derive::HelloMacroTrait;

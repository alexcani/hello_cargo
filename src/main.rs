// Declares module garden
// Code is inside src/garden.rs or src/garden/mod.rs
pub mod garden;

fn main() {
    // Package = top level, cargo feature that contains one or more crates
    // and their relationships. Has a Cargo.toml file
    // Crate = binary or library (essentially cmake targets?)
    // Packages can have as many binary crates as wanted but only 1 library crate

    // src/main.rs -> crate root of binary crate of same name as package
    // src/lib.rs -> crate root of library crate of same name as package
    // Can have both at the same time

    // For multiple binary crates, place files in src/bin directory

    use garden::vegetables::Asparagus;
    println!("Here is an Aspargus {:#?}", Asparagus{});

    // Call to library code of same name as package
    hello_cargo::eat_at_restaurant();
}


// When an enum is public all variants are public
pub enum Test {
    VariantA,
    VariantB(String)
}

// When a struct is public each field needs to be marked public on a per-case basis
pub struct TestStruct {
    pub fieldA: String,
    fieldB_is_private: u32
}

fn main() {
    // Package = top level, cargo feature that contains one or more crates
    // and their relationships. Has a Cargo.toml file
    // Crate = binary or library (essentially cmake targets?)
    // Packages can have as many binary crates as wanted but only 1 library crate

    // src/main.rs -> crate root of binary crate of same name as package
    // src/lib.rs -> crate root of library crate of same name as package
    // Can have both at the same time

    // For multiple binary crates, place files in src/bin directory
    println!("Hello, world!");
}

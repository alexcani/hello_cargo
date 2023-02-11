fn main() {
    // 4 types of macros:
    // macro_rules!
    // Custom #[derive] macros
    // Attribute-like macros #[some_attrib]
    // Function-like macros my_func!

    // Macros needs to be define and brought into scope before being used

    // Declarative macros with macro_rules!
    #[macro_export]  // makes the macro available when the crate is brought into scope
    macro_rules! my_vec {
        ($($x:expr), *) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        }
    }

    let x = my_vec![1, 2, 3];
    println!("{:?}", x);

    // https://veykril.github.io/tlborm/ The Little Book of Rust Macros


    // Procedural macros fro generating code from attributes
    /*use proc_macro;
    #[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {

    }*/

    use my_lib::HelloMacroTrait;
    use my_lib::Teste;
    //use my_lib_derive::HelloMacroTrait;

    #[derive(HelloMacroTrait)]
    struct Pancakes;

    Pancakes::hello_macro_fn();
    Pancakes::ha();


    // Attribute-like macros
    // #[route(GET, "/")]  for example
    fn index() {}

    // Works the same way, definition must be inside a
    // proc-macro crate
    // Check my_lib_derive lib.src


    // Function-like macros
    // Similar to declarative macro when calling but instead of following the macro_rules! matching syntax
    // it works like a proc-macro with TokenStream
    // Example is
    // sql!(SELECT * FROM posts WHERE x > 1)
    // Which can parse sql code and check for syntax validity among other things
    // Definition must be inside proc-macro crate, check my_lib_derive
}

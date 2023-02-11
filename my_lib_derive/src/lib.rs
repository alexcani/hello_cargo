use proc_macro::TokenStream;
use quote::quote;
use syn::{self, token::Token};

#[proc_macro_derive(HelloMacroTrait)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct repr. of rust code as syntax tree that we can manip.
    let ast = syn::parse(input).unwrap();

    // Trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacroTrait for #name {
            fn hello_macro_fn() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }

        impl Teste for #name {
            fn ha() {
                println!("haha");
            }
        }
    };
    gen.into()
}


// Attribute-like macro

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr is the content the macro is called with
    // item is the item the macro is attached to
    TokenStream::new()
}

// Example
// #[route(GET, "/")]
fn index() {}

// GET, "/" is attr
// fn index is the item



// Function-like macro
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {  // generic procedural macro
    TokenStream::new()
}

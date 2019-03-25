extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

// #[proc_macro]
// pub fn new_china_mobile_request(ts: TokenStream) -> TokenStream {
//     let map = ts.into_iter()
//     .count().to_string().parse().unwrap()
// }


#[proc_macro_derive(New)]
pub fn new(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = dbg!(&ast.ident);
    let output = quote! {
        impl #name {
            pub fn new_test() -> () {
                // dbg!(#name);
                // dbg!(#attrs);
                println!("Hello World from {}!", stringify!(#name));
            }
        }
    };
    output.into()
}

// #![feature(proc_macro_quote)]
extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::{token::Token, DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();
    let ident = &ast.ident;
    eprintln!("{:?}", ident);
    let gen = quote! {
        // struct #ident~Builder {

        // }
        impl Builder for #ident {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#ident));
            }
        }
    };
    return gen.into();

}
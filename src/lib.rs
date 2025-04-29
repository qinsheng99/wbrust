// extern crate proc_macro;
extern crate syn;

use quote::quote;
use syn::{DeriveInput, Lit, parse_macro_input};
use syn::__private::TokenStream;

// use proc_macro::TokenStream;

#[proc_macro_derive(MyDerive)]
pub fn proc_macro_handle(_item: TokenStream) -> TokenStream {
    "fn answer1() -> u32 { 429 }".parse().unwrap()
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 429 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn show_streams(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("attr: \"{}\"", attr.to_string());
    // println!("item: \"{}\"", item.to_string());
    item
}
#[proc_macro_derive(hm)]
pub fn hm(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HM for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(seac, attributes(seac_f))]
pub fn seactf(item: TokenStream) -> TokenStream {
    let d = parse_macro_input!(item as DeriveInput);

    if d.ident != "Model" {
        panic!("struct name must be Model,No:{:?}", d.ident.to_string())
    }

    #[allow(dead_code)]
    let struct_q = quote! {};

    d.attrs
        .iter()
        .filter(|attr| attr.path().is_ident("seac_f"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("comment") {
                    let name: Lit = meta.value()?.parse()?;
                    // println!("{:?}", Some(name));
                    if let Lit::Str(_s) = name {
                        // println!("{:?}", s);
                    }
                    // struct_q = quote! {
                    //     fn struct_f()->Option(&str) {
                    //         Some(#name)
                    //     }
                    // };
                }

                Ok(())
            })
        })
        .unwrap();

    struct_q.into()
}

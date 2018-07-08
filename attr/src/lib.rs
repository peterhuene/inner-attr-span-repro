#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::{Span, TokenStream};
use syn::{parse, ItemFn, Path};

fn to_string(path: &Path) -> String {
    let mut s = String::new();

    for segment in path.segments.iter() {
        if !s.is_empty() {
            s += "::";
        }

        s += &segment.ident.to_string();
    }

    s
}

#[proc_macro_attribute]
pub fn repro(_: TokenStream, input: TokenStream) -> TokenStream {
    // Print the input token stream to see the invalid span information
    println!("{:?}", input);

    let mut target: ItemFn = match parse(input.clone()) {
        Ok(f) => f,
        _ => {
            Span::call_site()
                .error("the 'repro' attribute can only be used on functions")
                .emit();
            return input;
        }
    };

    // Strip off the magic macros from the output
    target.attrs.retain(|x| to_string(&x.path) != "magic");

    quote!(#target).into()
}

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemMacro, Visibility};

pub fn expand(args: &Visibility, item: &ItemMacro) -> TokenStream {
    let ident = &item.ident;

    quote! {
        #item

        #args use #ident;
    }
}

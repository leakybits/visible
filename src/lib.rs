//! `visible` makes a macro visible within a given scope.

#[macro_use]
extern crate proc_util;

/// Implements the `#[visible(...)]` macro.
mod visible;

/// Makes a macro visible within a given scope.
#[proc_macro_attribute]
pub fn visible(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    render!(args, item, { visible::expand(&args, &item) })
}

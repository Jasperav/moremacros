extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod db_mirror;

use proc_macro::TokenStream;
use db_mirror::generate_select_queries;

#[proc_macro_derive(DBMirror, attributes(partition_key, clustering_key))]
pub fn db_mirror(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = generate_select_queries(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
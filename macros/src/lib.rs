use proc_macro::TokenStream as StdTokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input};

use crate::parser::MixedRadixInfo;

mod parser;

struct MultipleMRI(Vec<MixedRadixInfo>);

impl Parse for MultipleMRI {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut infos = Vec::new();

    while !input.is_empty() {
      infos.push(input.parse()?);
    }

    Ok(Self(infos))
  }
}

#[proc_macro]
/// Mixed Radix Generator
///
/// This is a proc_macro that has the following syntax to generate Mixed Radix Representation
///
/// ```rust
/// mixedradix::mixedradix! {
///   #[bits(7)] // <-- Use 7 bits (i.e. at max 128 states supported)
///   #[derive(....)] // <-- Your custom attributes
///   pub struct MyBitsStruct {
///     #[doc = "Represents the primary action button."]
///     pub a: 7, // <-- A has total `7` states
///   }
/// }
/// ```
///
/// # Implements
///
/// Th
///
/// ## States Formula
/// To get the number of states for a `#[bits(N)]`
///
/// States = `2^N`
///
/// ## Finding used states
///
/// Multiply the states of each field to get the used state.
pub fn mixedradix(input: StdTokenStream) -> StdTokenStream {
  let parsed = parse_macro_input!(input as MultipleMRI).0;

  quote! {
    #(#parsed)*
  }
  .into()
}

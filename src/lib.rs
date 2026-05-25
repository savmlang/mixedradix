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
pub fn mixedradix(input: StdTokenStream) -> StdTokenStream {
  let parsed = parse_macro_input!(input as MultipleMRI).0;

  quote! {
    #(#parsed)*
  }
  .into()
}

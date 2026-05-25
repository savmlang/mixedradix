use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
  Attribute, Error, Ident, LitInt, Meta, Token, Visibility, braced, parse::Parse,
  punctuated::Punctuated, spanned::Spanned,
};

pub mod kw {
  use syn::custom_keyword;

  custom_keyword!(bits);
}
pub struct MixedRadixInfo {
  vis: Visibility,
  bits: u8,
  bitstype: TokenStream,
  attrs: Vec<Attribute>,
  name: Ident,
  fields: Punctuated<RadixField, Token![,]>,
}

impl ToTokens for MixedRadixInfo {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let bits = self.bits;
    let bitstype = &self.bitstype;
    let attrs = &self.attrs;
    let vis = &self.vis;
    let name = &self.name;

    let max_value = self.fields.iter().fold(1u128, |curr, t| curr * t.states) - 1;

    let fields = self.fields.iter().map(|x| {
      let vis = &x.vis;
      let name = &x.name;
      let ty = &x.re_type;
      let attr = &x.attr;
      quote! {
        #(#attr)*
        #vis #name: #ty
      }
    });

    let field_construct = self.fields.iter().map(|x| {
      let name = &x.name;

      quote! {
        #name: #name as _
      }
    });
    let field_construct = field_construct.collect::<Vec<_>>().clone();
    let field_construct2 = field_construct.clone();

    let mut weight: u128 = 1;
    let weights = self.fields.iter().map(|x| {
      x.states;
      let o = weight;
      weight *= x.states;
      o
    });

    let ((ser, ser2), de): ((Vec<_>, Vec<_>), Vec<_>) = weights
      .zip(self.fields.iter())
      .map(|(weight, curr)| {
        let name = &curr.name;
        let states = curr.states;

        let panic_msg = format!("Mixed-radix serialization overflow for field `{name}`");

        let ser = quote! {
          debug_assert!(
            (self.#name as u128) < #states,
            #panic_msg
          );
          total += (self.#name as #bitstype) * (#weight as #bitstype);
        };
        let ser2 = quote! {
          if (self.#name as u128) >= #states {
            return None;
          }
          total += (self.#name as #bitstype) * (#weight as #bitstype);
        };
        let de = quote! {
          let #name = (total / (#weight as #bitstype)) % (#states as #bitstype);
        };

        ((ser, ser2), de)
      })
      .unzip();

    let out = quote! {
      #(#attrs)*
      #vis struct #name {
        #(#fields),*
      }

      impl mixedradix::MixedRadixStructure for #name {
        type BitsType = #bitstype;

        const STORAGE_BITS: u8 = #bits;
        const MAXIMUM_VALUE: #bitstype = #max_value as #bitstype;

        fn bits(&self) -> #bitstype {
          let mut total: #bitstype = 0;

          #(#ser)*

          total
        }

        fn try_bits(&self) -> Option<#bitstype> {
          let mut total: #bitstype = 0;

          #(#ser2)*

          Some(total)
        }

        fn from_bits(total: #bitstype) -> Self {
          debug_assert!(total <= (#max_value as _), "Overflow has been detected. Please ensure your total is not corrupted.");
          #(#de)*

          Self {
            #(#field_construct),*
          }
        }

        fn try_from_bits(total: #bitstype) -> Option<Self> {
          if total > (#max_value as _) {
            return None;
          }

          #(#de)*

          Some(Self {
            #(#field_construct2),*
          })
        }
      }
    };

    tokens.extend(out);
  }
}

impl Parse for MixedRadixInfo {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut attrs = input.call(Attribute::parse_outer)?;
    attrs.extend(input.call(Attribute::parse_inner)?);

    let vis = input.parse::<Visibility>()?;
    input.parse::<Token![struct]>()?;
    let name = input.parse::<Ident>()?;

    let body_content;
    braced!(body_content in input);
    let fields = body_content.parse_terminated(RadixField::parse, Token![,])?;

    let mut span = None;
    let mut bits = Err(Error::new(
      input.span(),
      "#[bits($value)] attribute was not found.",
    ));

    attrs.retain_mut(|x| {
      if x.path().is_ident("bits") {
        if bits.is_ok() {
          bits = Err(Error::new(
            input.span(),
            "Multiple #[bits] argument were found!",
          ));
        }

        let Meta::List(lst) = &x.meta else {
          bits = Err(Error::new(
            x.span(),
            "Invalid `#[bits]` syntax, expected #[bits(bits_int)]",
          ));
          return true;
        };

        span = Some(lst.span());
        let parsedint = match lst
          .parse_args::<LitInt>()
          .and_then(|x| x.base10_parse::<u8>())
        {
          Err(e) => {
            bits = Err(e);
            return true;
          }
          Ok(val) => val,
        };

        bits = Ok(parsedint);
        return false;
      }

      true
    });

    let bits = bits?;

    let max_states = 2u128.pow(bits as _);
    let total_states = fields.iter().fold(1u128, |v, x| v.saturating_mul(x.states));

    if total_states > max_states {
      return Err(Error::new(
        span.unwrap(),
        format!(
          "Overflow - representable states ({total_states}) is greater than total possible states ({max_states})"
        ),
      ));
    }

    let bitstype = match bits {
      1..=8 => quote! { u8 },
      9..=16 => quote! { u16 },
      17..=32 => quote! { u32 },
      33..=64 => quote! { u64 },
      0 => {
        return Err(Error::new(span.unwrap(), "0 bits are not possible"));
      }
      _ => {
        return Err(Error::new(
          span.unwrap(),
          "Bit overflow - we do not support bitpacking above 64 bits",
        ));
      }
    };

    Ok(Self {
      attrs,
      fields,
      name,
      vis,
      bits,
      bitstype,
    })
  }
}

pub struct RadixField {
  vis: Visibility,
  attr: Vec<Attribute>,
  name: Ident,
  states: u128,
  re_type: TokenStream,
}

impl Parse for RadixField {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut attr = input.call(Attribute::parse_outer)?;
    attr.extend(input.call(Attribute::parse_inner)?);

    let vis = input.parse::<Visibility>()?;

    let name = input.parse::<Ident>()?;
    input.parse::<Token![:]>()?;
    let totalstates = input.parse::<LitInt>()?;

    let states = totalstates.base10_parse::<u128>()?;

    if states > 2u128.pow(64) {
      return Err(Error::new(
        totalstates.span(),
        "Field states exceed u64 maximum",
      ));
    }

    let re_type = match states {
      1 => {
        return Err(Error::new(
          totalstates.span(),
          "Single state is not allowed.",
        ));
      }
      2..=256 => quote! { u8 },
      257..=65536 => quote! { u16 },
      65537..=4294967296 => quote! { u32 },
      _ => quote! { u64 },
    };

    Ok(Self {
      attr,
      vis,
      name,
      states,
      re_type,
    })
  }
}

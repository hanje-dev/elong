use pmutil::{smart_quote, Quote};
use swc_macros_common::prelude::*;
use syn::{
  self,
  parse::{Parse, ParseStream},
  *,
};

#[derive(Clone)]
pub struct Args {
  pub ty: Literal,
}

impl Parse for Args {
  fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
    Ok(Args { ty: input.parse()? })
  }
}

pub fn expand_struct(args: Args, i: DeriveInput) -> Vec<ItemImpl> {
  let mut items = vec![];
  let generics = i.generics.clone();

  items.push(
    Quote::new_call_site()
      .quote_with(smart_quote!(
        Vars {
          Type: i.ident.clone(),
          type_str: args.ty
        },
        {
          impl ::swc_common::AstNode for Type {
            const TYPE: &'static str = type_str;
          }
        }
      ))
      .parse::<ItemImpl>()
      .with_generics(generics),
  );

  items
}

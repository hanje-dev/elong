#![deny(clippy::all)]
#![recursion_limit = "1024"]

extern crate proc_macro;

use pmutil::{smart_quote, Quote, ToTokensExt};
use swc_macros_common::prelude::*;
use syn::{self, visit_mut::VisitMut, *};

mod ast_node_macro;
mod enum_deserialize;
mod spanned;

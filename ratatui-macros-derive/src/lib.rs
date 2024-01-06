use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::proc_macro_error;
use quote::quote;
use ratatui_macros_core::style_line_core;
use syn::{parse_macro_input, LitStr};

#[proc_macro_error]
#[proc_macro]
pub fn style_line(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as LitStr);
  style_line_core(input.value()).into()
}

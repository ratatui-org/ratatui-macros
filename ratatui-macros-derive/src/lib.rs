use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro]
pub fn style_format(input: TokenStream) -> TokenStream {
  style_format_core(input.into()).into()
}

fn style_format_core(input: TokenStream2) -> TokenStream2 {
  input
}

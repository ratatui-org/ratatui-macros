use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro_error]
#[proc_macro]
pub fn style_line(input: TokenStream) -> TokenStream {
  style_line_core(input.into()).into()
}

fn style_line_core(input: TokenStream2) -> TokenStream2 {
  dbg!(input);
  quote! {
    ()
  }
}

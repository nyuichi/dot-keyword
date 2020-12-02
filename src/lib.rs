use proc_macro::*;

#[proc_macro]
pub fn postfix_match(input: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(input).unwrap();
    quote::quote!(#expr).into()
}

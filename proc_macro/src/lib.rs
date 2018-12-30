extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

#[proc_macro]
pub fn forward(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("input: {:?}", tokens);

    let test = syn::parse_macro_input!(tokens as syn::Ident);

    let res = quote::quote!(
        #test
    );

    let res = res.into();
    println!("output: {:?}", res);
    res
}

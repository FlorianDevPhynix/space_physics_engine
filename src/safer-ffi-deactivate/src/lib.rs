use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn ffi_export (
    attrs: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    parse_macro_input!(attrs as syn::parse::Nothing);
    if cfg!(feature = "ffi_compile") {
        format!("#[::safer_ffi::ffi_export]\n{}", input).parse().unwrap()
    } else {
        input
    }
}

#[proc_macro_attribute]
pub fn derive_ReprC (
    attrs: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    //eprintln!("attrs: \"{}\"", attrs);
    if cfg!(feature = "ffi_compile") {
        format!("#[::safer_ffi::derive_ReprC]\n#[repr({})]\n{}", attrs, input).parse().unwrap()
    } else {
        input
    }
}
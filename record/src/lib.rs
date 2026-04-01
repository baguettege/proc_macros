use proc_macro::TokenStream;

mod expand;

#[proc_macro_derive(Record, attributes(record))]
pub fn record_derive(input: TokenStream) -> TokenStream {
    record_derive_inner(input).unwrap_or_else(
        |e| e.to_compile_error().into())
}

fn record_derive_inner(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input = syn::parse::<syn::DeriveInput>(input)?;
    let def = ir::StructDef::try_from(input)?;
    Ok(expand::expand(&def)?.into())
}

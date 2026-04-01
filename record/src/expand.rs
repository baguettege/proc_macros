use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Meta;
use ir::{FieldDef, StructDef, StructKind};

/// Expands `def` into a record.
pub(crate) fn expand(def: &StructDef) -> Result<TokenStream, syn::Error> {
    let new_fn = expand_new_fn(def);
    let getters = expand_getters(def)?;
    let ident = def.ident();
    
    let (impl_generics, type_generics, where_clause) =
        def.generics().split_for_impl();

    let ts = quote! {
        impl #impl_generics #ident #type_generics #where_clause {
            #new_fn
            #getters
        }
    };

    Ok(ts)
}

/// Expands `def` into a `new` function for each of its fields.
///
/// # Panics
/// If `def` has a kind `StructKind::Named` with unnamed fields. More specifically,
/// if any one of these fields `ident` is None.
fn expand_new_fn(def: &StructDef) -> TokenStream {
    match def.kind() {
        StructKind::Unit => {
            quote! {
                pub fn new() -> Self {
                    Self
                }
            }
        }
        StructKind::Named(fields) => {
            let (params, idents): (Vec<_>, Vec<_>) = fields
                .iter()
                .map(|f| {
                    let ident = f.ident().as_ref().unwrap();
                    let ty= f.ty();
                    (quote!( #ident: #ty ), quote!( #ident ))
                })
                .unzip();

            quote! {
                pub fn new( #( #params ),* ) -> Self {
                    Self { #( #idents ),* }
                }
            }
        }
        StructKind::Tuple(fields) => {
            let (params, idents): (Vec<_>, Vec<_>) = fields
                .iter()
                .enumerate()
                .map(|(idx, f)| {
                    let ident = format_ident!("field_{}", idx);
                    let ty = f.ty();
                    (quote!( #ident: #ty ), quote!( #ident ))
                })
                .unzip();

            quote! {
                pub fn new( #( #params ),* ) -> Self {
                    Self { #( #idents ),* }
                }
            }
        }
    }
}

/// Expands `def` into getter methods for each of its fields.
fn expand_getters(def: &StructDef) -> Result<TokenStream, syn::Error> {
    match def.kind() {
        StructKind::Unit => Ok(TokenStream::new()),
        StructKind::Named(fields) => {
            let mut getters = Vec::new();

            for field in fields.iter() {
                let ts = gen_named_getter(field)?;
                getters.push(ts);
            }

            Ok(quote!( #( #getters )* ))
        }
        StructKind::Tuple(fields) => {
            let mut getters = Vec::new();

            for (index, field) in fields.iter().enumerate() {
                let ts = gen_unnamed_getter(field, index);
                getters.push(ts);
            }

            Ok(quote!( #( #getters )* ))
        }
    }
}

/// Returns whether `field` has the attribute `#[record(copy)]`.
fn has_copy_attr(field: &FieldDef) -> Result<bool, syn::Error> {
    let mut should_copy = false;

    for attr in field.attrs() {
        if attr.path().is_ident("record") {
            match attr {
                Meta::List(list) => {
                    let ts = &list.tokens;
                    let path = syn::parse2::<syn::Path>(ts.clone())?;

                    if path.is_ident("copy") {
                        should_copy = true;
                    } else {
                        let err = syn::Error::new_spanned(
                            path,
                            "unknown attribute"
                        );
                        return Err(err);
                    }
                }
                _ => {
                    let err = syn::Error::new_spanned(
                        attr,
                        "record attribute must be a list"
                    );
                    return Err(err);
                }
            }
        }
    }

    Ok(should_copy)
}

/// Generates a getter for `field` with the format `ident`.
///
/// # Panics
/// If `field` is an unnamed field. More specifically, if `field.ident`
/// is None.
fn gen_named_getter(field: &FieldDef) -> Result<TokenStream, syn::Error> {
    let ident = field.ident().as_ref().unwrap();
    let ty = field.ty();

    let ts = if has_copy_attr(&field)? {
        quote! {
            pub fn #ident(&self) -> #ty {
                self.#ident
            }
        }
    } else {
        quote! {
            pub fn #ident(&self) -> &#ty {
                &self.#ident
            }
        }
    };

    Ok(ts)
}

/// Generates a getter for `field`, for the specified `index`, with
/// the format `get_index`.
fn gen_unnamed_getter(field: &FieldDef, index: usize) -> TokenStream {
    let ident = format_ident!("get_{}", index);
    let ty = field.ty();

    let idx = syn::Index::from(index);

    quote! {
        pub fn #ident(&self) -> &#ty {
            &self.#idx
        }
    }
}

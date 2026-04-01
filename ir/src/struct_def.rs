use syn::{DeriveInput, Fields};
use crate::field_def::FieldDef;
use crate::util::record;

pub enum StructKind {
    Unit,
    Named(Vec<FieldDef>),
    Tuple(Vec<FieldDef>),
}

record! {
    StructDef {
        ident: syn::Ident,
        generics: syn::Generics,
        kind: StructKind,
    }
}

impl TryFrom<DeriveInput> for StructDef {
    type Error = syn::Error;

    fn try_from(input: DeriveInput) -> Result<Self, Self::Error> {
        let ident = input.ident;
        let generics = input.generics;

        let fields = match input.data {
            syn::Data::Struct(data_struct) => data_struct.fields,
            _ => {
                let err = syn::Error::new_spanned(
                    &ident, "expected struct");
                return Err(err);
            }
        };

        let kind = match fields {
            Fields::Named(named) => {
                let parsed = named.named
                    .into_iter()
                    .map(FieldDef::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                StructKind::Named(parsed)
            }
            Fields::Unnamed(unnamed) => {
                let parsed = unnamed.unnamed
                    .into_iter()
                    .map(FieldDef::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                StructKind::Tuple(parsed)
            }
            Fields::Unit => StructKind::Unit
        };

        Ok(StructDef::new(ident, generics, kind))
    }
}

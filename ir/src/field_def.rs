use syn::Field;
use crate::util::record;

record! {
    FieldDef {
        ident: Option<syn::Ident>,
        ty: syn::Type,
        attrs: Vec<syn::Meta>,
    }
}

impl TryFrom<Field> for FieldDef {
    type Error = syn::Error;

    fn try_from(field: Field) -> Result<Self, Self::Error> {
        let name = field.ident;
        let ty = field.ty;
        let attrs = field.attrs.into_iter().map(|a| a.meta).collect();

        Ok(FieldDef::new(name, ty, attrs))
    }
}

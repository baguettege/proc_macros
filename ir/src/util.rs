macro_rules! record {
    (
        $ident:ident {
            $(
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        pub struct $ident {
            $( $field: $ty ),*
        }

        impl $ident {
            pub fn new( $( $field: $ty ),* ) -> Self {
                Self { $( $field ),* }
            }

            $(
                pub fn $field(&self) -> &$ty {
                    &self.$field
                }
            )*
        }
    };
}

pub(crate) use record;

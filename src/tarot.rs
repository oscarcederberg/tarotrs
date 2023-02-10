pub mod card;
pub mod spread;
pub mod deck;

#[macro_export]
macro_rules! enum_try_from {
    (
        #[repr($T: ident)]
        $( #[$meta: meta] )*
        $vis: vis enum $Name: ident {
            $(
                $Variant: ident = $value: expr
            ),*
            $( , )?
        }
    ) => {
        #[repr($T)]
        $( #[$meta] )*
        $vis enum $Name {
            $(
                $Variant = $value
            ),*
        }

        impl std::convert::TryFrom<$T> for $Name {
            type Error = ();

            fn try_from(value: $T) -> Result<$Name, ()> {
                match value {
                    $(
                        $value => Ok($Name::$Variant),
                    )*
                    _ => Err(())
                }
            }
        }
    }
}

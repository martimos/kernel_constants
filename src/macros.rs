/// Generates a TryFrom implementation for the enclosed enum.
///
///     generate_try_from! {
///         enum MyEnum: usize {
///             ...
///         }
///     }
///
/// will generate a TryFrom&lt;usize&gt; implementation for MyEnum.
#[macro_export]
macro_rules! generate_try_from {
    ($(#[$meta:meta])* $vis:vis enum $name:ident : $typ:ty {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl core::convert::TryFrom<$typ> for $name {
            type Error = ();

            fn try_from(v: $typ) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as $typ => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

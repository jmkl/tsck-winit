#[macro_export]
macro_rules! generate_func_enums {
    (
        $entry_enum:ident => (
            $(
                $entry_variant:ident => (
                    $($func_variant:ident),* $(,)?
                )
            )*
        )
    ) => {
        // Generate the main Entry enum
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $entry_enum {
            $(
                $entry_variant,
            )*
        }

        // Generate FromStr for Entry
        impl std::str::FromStr for $entry_enum {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s_lower = s.to_lowercase();
                match s_lower.as_str() {
                    $(
                        _ if s_lower == stringify!($entry_variant).to_lowercase() => {
                            Ok(Self::$entry_variant)
                        }
                    )*
                    _ => Err(format!("Unknown entry: {}", s))
                }
            }
        }

        // Generate function enums for each entry
        $(
            $crate::paste::paste! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum [<$entry_variant Func>] {
                    $(
                        $func_variant,
                    )*
                }

                impl std::str::FromStr for [<$entry_variant Func>] {
                    type Err = String;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        let s_upper = s.to_uppercase();
                        match s_upper.as_str() {
                            $(
                                _ if s_upper == stringify!($func_variant).to_uppercase() => {
                                    Ok(Self::$func_variant)
                                }
                            )*
                            _ => Err(format!("Unknown {} function: {}", stringify!($entry_variant), s))
                        }
                    }
                }
            }
        )*
    };
}

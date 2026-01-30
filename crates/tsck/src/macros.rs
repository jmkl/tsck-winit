#[macro_export]
macro_rules! ts_struct {
    // With extra derives + custom Serialize impl
    (path = $path:expr, ($($extra:ident),+), impl_serialize, $(#[$attr:meta])* $item:item) => {
        #[derive(Debug, Clone, Deserialize, PartialEq, TS, $($extra),+)]
        #[ts(export, export_to = $path)]
        $(#[$attr])*
        $item
    };

    // With extra derives
    (path = $path:expr, ($($extra:ident),+), $(#[$attr:meta])* $item:item) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS, $($extra),+)]
        #[ts(export, export_to = $path)]
        $(#[$attr])*
        $item
    };

    // With custom Serialize impl
    (path = $path:expr, impl_serialize, $(#[$attr:meta])* $item:item) => {
        #[derive(Debug, Clone, Deserialize, PartialEq, TS)]
        #[ts(export, export_to = $path)]
        $(#[$attr])*
        $item
    };

    // With attributes only
    (path = $path:expr, $(#[$attr:meta])+ $item:item) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
        #[ts(export, export_to = $path)]
        $(#[$attr])+
        $item
    };

    // Basic usage (default)
    (path = $path:expr, $item:item) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
        #[ts(export, export_to = $path)]
        $item
    };
}

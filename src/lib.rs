pub mod color;
pub mod logger;

// Re-export indexmap for macro use
pub use indexmap;

#[macro_export]
macro_rules! table_data {
    ($($key:ident : $value:expr),* $(,)?) => {{
        let mut map = $crate::indexmap::IndexMap::new();
        $(
            let value_str = format!("{}", $value);
            map.insert(stringify!($key), value_str);
        )*
        map
    }};
}

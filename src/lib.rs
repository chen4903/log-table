pub mod logger;

#[macro_export]
macro_rules! table_data {
    ($($key:ident : $value:expr),* $(,)?) => {{
        let mut map = ::indexmap::IndexMap::new();
        $(
            map.insert(stringify!($key), $value.to_string());
        )*
        map
    }};
}
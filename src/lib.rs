pub mod logger;

#[macro_export]
macro_rules! table_data {
    ($($key:ident : $value:expr),* $(,)?) => {{
        let mut map = ::indexmap::IndexMap::new();
        $(
            let value_str = format!("{}", $value);
            map.insert(stringify!($key), value_str);
        )*
        map
    }};
}
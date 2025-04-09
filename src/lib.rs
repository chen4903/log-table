pub mod logger;

#[macro_export]
macro_rules! table_data {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value.to_string());
        )*
        map
    }};
}
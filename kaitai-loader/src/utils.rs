#[macro_export]
macro_rules! hash_map {
    {} => {::std::collections::HashMap::new()};
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
}

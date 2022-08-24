#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr, )+ ) => { hashmap!($($key => $value),+) };
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut tmp_hashmap = ::std::collections::HashMap::new();

            $(
                tmp_hashmap.insert({$key}, {$value});
            )*

            tmp_hashmap
        }
    };
}

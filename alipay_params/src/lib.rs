pub use alipay_macros::*;
use std::collections::HashMap;

pub trait PublicParams {
    fn to_hash_map(&self) -> HashMap<String, String>;
}

impl<T1, T2> PublicParams for (T1, T2)
where
    T1: ToString,
    T2: ToString,
{
    fn to_hash_map(&self) -> HashMap<String, String> {
        HashMap::from([(self.0.to_string(), self.1.to_string())])
    }
}

impl<T1, T2, const N: usize> PublicParams for [(T1, T2); N]
where
    T1: ToString,
    T2: ToString,
{
    fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for val in self {
            map.insert(val.0.to_string(), val.1.to_string());
        }
        map
    }
}

impl<T1, T2> PublicParams for Vec<(T1, T2)>
where
    T1: ToString,
    T2: ToString,
{
    fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for val in self {
            map.insert(val.0.to_string(), val.1.to_string());
        }
        map
    }
}

impl<T1, T2> PublicParams for HashMap<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (key, val) in self {
            map.insert(key.to_string(), val.to_string());
        }
        map
    }
}

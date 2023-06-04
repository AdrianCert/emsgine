use crate::models::bytes::DataWordSized;

#[macro_export]
macro_rules! safe_lookup {
    ($tbl:expr, $key:literal) => {
        $tbl.lookup($key).unwrap_or_default()
    };
}

pub trait Lookup<K,V> {
    fn lookup(&self, key: K) -> Option<&V>;
}

impl<K,V> Lookup<K,V> for std::collections::HashMap<K,V>
where
    K: std::cmp::Eq + core::hash::Hash
{
    fn lookup(&self, key: K) -> Option<&V> {
        self.get(&key)
    }
}

impl<'b> Lookup<&str, DataWordSized> for Vec<(&'b str, DataWordSized)> {
    fn lookup(&self, tkey: &str) -> Option<&DataWordSized> {
        for pair in self.iter() {
            let (key, value) = pair;
            if *key == tkey {
                return Some(value);
            }
        }
        None
    }
}

pub use safe_lookup;

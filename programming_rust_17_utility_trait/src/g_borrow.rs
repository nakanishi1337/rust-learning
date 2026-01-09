use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Weird(String);

impl AsRef<str> for Weird {
    fn as_ref(&self) -> &str {
        &self.0[..1]
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Normal(String);

impl Borrow<str> for Normal {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demonstrate_asref_weakness() {
        let a = Weird("hello".into());
        let b = Weird("hi".into());

        assert_ne!(a, b);
        assert_eq!(a.as_ref(), b.as_ref()); // "h"
    }

    #[test]
    fn demonstrate_borrow_strength() {
        let a = Normal("hello".into());
        let b = Normal("hello".into());

        let a_str: &str = a.borrow();
        let b_str: &str = b.borrow();

        assert_eq!(a_str, b_str);
    }

    #[test]
    fn hashmap_uses_borrow_contract() {
        let mut map: HashMap<Normal, i32> = HashMap::new();
        map.insert(Normal("hello".into()), 1);

        assert_eq!(map.get("hello"), Some(&1));
    }
}

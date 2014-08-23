#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! A type-based key value store where one value type is allowed for each key.

use std::any::{Any, AnyRefExt, AnyMutRefExt};
use std::intrinsics::TypeId;
use std::collections::HashMap;

/// A map keyed by types.
///
/// Can contain one value of any type for each key type, as defined
/// by the Assoc trait.
pub struct TypeMap {
    data: HashMap<TypeId, Box<Any>>
}

/// This trait defines the relationship between keys and values in a TypeMap.
///
/// It is implemented for Keys, with a phantom type parameter for values.
pub trait Assoc<Value> {}

impl TypeMap {
    /// Create a new, empty TypeMap.
    pub fn new() -> TypeMap {
        TypeMap {
            data: HashMap::new()
        }
    }

    /// Insert a value into the map with a specified key type.
    pub fn insert<K: Assoc<V> + 'static, V: 'static>(&mut self, _key: K, val: V) -> bool {
        self.data.insert(TypeId::of::<K>(), box val as Box<Any>)
    }

    /// Find a value in the map and get a reference to it.
    pub fn find<K: Assoc<V> + 'static, V: 'static>(&self, _key: K) -> Option<&V> {
        self.data.find(&TypeId::of::<K>()).and_then(|v| v.downcast_ref::<V>())
    }

    /// Find a value in the map and get a mutable reference to it.
    pub fn find_mut<K: Assoc<V> + 'static, V: 'static>(&mut self, _key: K) -> Option<&mut V> {
        self.data.find_mut(&TypeId::of::<K>()).and_then(|v| v.downcast_mut::<V>())
    }

    /// Check if a key has an associated value stored in the map.
    pub fn contains<K: Assoc<V> + 'static, V: 'static>(&self, _key: K) -> bool {
        self.data.contains_key(&TypeId::of::<K>())
    }

    /// Remove a value from the map.
    ///
    /// Returns `true` if a value was removed.
    pub fn remove<K: Assoc<V> + 'static, V: 'static>(&mut self, _key: K) -> bool {
        self.data.remove(&TypeId::of::<K>())
    }
}

impl Collection for TypeMap {
    fn len(&self) -> uint {
        self.data.len()
    }
}

impl Mutable for TypeMap {
    fn clear(&mut self) {
        self.data.clear()
    }
}

#[cfg(test)]
mod test {
    use super::{TypeMap, Assoc};

    #[deriving(Show, PartialEq)]
    struct Key;

    #[deriving(Show, PartialEq)]
    struct Value;

    impl Assoc<Value> for Key {}

    #[test] fn test_pairing() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Key, Value);
        assert_eq!(*map.find::<Key, Value>(Key).unwrap(), Value);
        assert!(map.contains::<Key, Value>(Key));
    }

    #[test] fn test_pairing_with_hashmap_syntax() {
        let mut map = TypeMap::new();
        map.insert(Key, Value);
        assert_eq!(*map.find::<Key, Value>(Key).unwrap(), Value);
        assert!(map.contains::<Key, Value>(Key));
    }

    #[test] fn test_finding_with_hashmap_syntax() {
        let mut map = TypeMap::new();
        map.insert(Key, Value);
        assert_eq!(*map.find(Key).unwrap(), Value);
    }

    #[test] fn test_contains_with_hashmap_syntax() {
        let mut map = TypeMap::new();
        map.insert(Key, Value);
        assert!(map.contains(Key));
    }

    #[test] fn test_remove() {
        let mut map = TypeMap::new();
        map.insert::<Key, Value>(Key, Value);
        assert!(map.contains::<Key, Value>(Key));
        map.remove::<Key, Value>(Key);
        assert!(!map.contains::<Key, Value>(Key));
    }

    #[test] fn test_remove_with_hashmap_syntax() {
        let mut map = TypeMap::new();
        map.insert(Key, Value);
        assert!(map.contains(Key));
        map.remove(Key);
        assert!(!map.contains(Key));
    }
}


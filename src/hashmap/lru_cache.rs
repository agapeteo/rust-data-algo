use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct CacheLru<'a, 'b, K: Eq + Hash, V> {
    cache: HashMap<&'a K, &'b V>,
    queue: VecDeque<&'a K>,
    capacity: usize,
}

#[allow(dead_code)]
impl<'a, 'b, K: Eq + Hash, V> CacheLru<'a, 'b, K, V> {
    pub fn new(capacity: usize) -> Self {
        CacheLru { cache: HashMap::new(), queue: VecDeque::new(), capacity }
    }

    pub fn set(&mut self, key: &'a K, value: &'b V) {
        if self.cache.len() >= self.capacity {
            let key_to_remove = self.queue.pop_front().unwrap();
            self.cache.remove_entry(key_to_remove);
        }
        self.cache.insert(key, value);
        self.queue.push_back(key);
    }

    pub fn get(&self, key: &'a K) -> Option<&'b V> {
        self.cache.get(key).copied()
    }
}

#[test]
fn test() {
    let mut cache: CacheLru<i32, &str> = CacheLru::new(3);

    assert_eq!(cache.get(&1), None);

    cache.set(&1, &"one");
    assert_eq!(cache.get(&1), Some(&"one"));
    assert_eq!(cache.get(&2), None);

    cache.set(&2, &"two");
    assert_eq!(cache.get(&1), Some(&"one"));
    assert_eq!(cache.get(&2), Some(&"two"));
    assert_eq!(cache.get(&3), None);

    cache.set(&3, &"three");
    assert_eq!(cache.get(&3), Some(&"three"));
    assert_eq!(cache.get(&4), None);

    cache.set(&4, &"four");
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&"two"));
    assert_eq!(cache.get(&3), Some(&"three"));
    assert_eq!(cache.get(&4), Some(&"four"));

    cache.set(&5, &"five");
    assert_eq!(cache.get(&5), Some(&"five"));
    assert_eq!(cache.get(&2), None);
}
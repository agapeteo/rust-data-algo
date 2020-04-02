mod word_count;

use std::collections::HashMap;

#[test]
fn hashmap() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&3), Some(&"three"));
    assert_eq!(map.get(&4), None);

    for (k, v) in map {
        println!("key: {}, value: {}", k, v);
    }
}

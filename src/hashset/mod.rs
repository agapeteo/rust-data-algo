use std::collections::HashSet;

#[test]
fn hashset() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(1);
    set.insert(2);
    set.insert(2);

    assert_eq!(set.len(), 2);
    assert_eq!(set.contains(&1), true);
    assert_eq!(set.contains(&2), true);
    set.remove(&2);
    assert_eq!(set.contains(&2), false);

    for n in set {
        println!("{}", n)
    }
}
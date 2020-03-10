use std::collections::VecDeque;

#[test]
fn vecdeque() {
    // new with capacity
    let _v: VecDeque<&str> = VecDeque::with_capacity(1_000);

    // from Vec
    let _v = VecDeque::from(vec![1, 2, 3]);

    // empty new
    let mut v: VecDeque<i32> = VecDeque::new();

    // add to end
    v.push_back(1);
    v.push_back(2);
    assert_eq!(v, [1, 2]);

    // pop last
    let last = v.pop_back();
    assert_eq!(Some(2), last);
    assert_eq!(v, [1]);


    // add to front beginning
    v.push_front(0);
    assert_eq!(v, [0, 1]);

    let first = v.pop_front();
    assert_eq!(Some(0), first);
    assert_eq!(v, [1]);

    // get by index and mutate
    if let Some(elem) = v.get_mut(0) {
        *elem *= 5;
    }

    // get by index
    assert_eq!(Some(&5), v.get(0));

    // iterate
    assert_eq!(Some(&5), v.iter().next());
}
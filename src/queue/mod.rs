use std::collections::VecDeque;

#[test]
fn queue() {
    let mut queue: VecDeque<u8> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);

    assert_eq!(queue.len(), 3);

    assert_eq!(queue.get(1), Some(&1));
    assert_eq!(queue.pop_front(), Some(0));
    assert_eq!(queue.pop_back(), Some(2));
    assert_eq!(queue.pop_back(), Some(1));
    assert_eq!(queue.pop_back(), None);
}
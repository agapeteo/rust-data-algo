use std::collections::VecDeque;

#[test]
fn stack() {
    let mut stack = VecDeque::new();
    stack.push_back(1);
    stack.push_back(2);
    stack.push_back(3);

    assert_eq!(stack.len(), 3);

    assert_eq!(stack.pop_back(), Some(3));
    assert_eq!(stack.pop_back(), Some(2));
    assert_eq!(stack.pop_back(), Some(1));
    assert_eq!(stack.pop_back(), None);
}
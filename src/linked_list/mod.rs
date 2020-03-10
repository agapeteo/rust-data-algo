use std::collections::LinkedList;

#[test]
fn linked_list() {
    let mut linked_list: LinkedList<u8> = LinkedList::new();
    assert_eq!(linked_list.is_empty(), true);

    linked_list.push_back(1);
    linked_list.push_back(2);
    linked_list.push_front(0);

    assert_eq!(linked_list.pop_front(), Some(0));
    assert_eq!(linked_list.pop_back(), Some(2));
    assert_eq!(linked_list.pop_back(), Some(1));
    assert_eq!(linked_list.pop_back(), None);
    assert_eq!(linked_list.is_empty(), true);
}

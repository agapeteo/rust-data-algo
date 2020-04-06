use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::collections::hash_map::Entry;
use std::cell::RefCell;

type NodeLink<K, V> = Rc<RefCell<Node<K, V>>>;

struct Node<K: Eq + Hash + Copy, V: Copy> {
    key: K,
    value: V,
    next: Option<NodeLink<K, V>>,
    prev: Option<NodeLink<K, V>>,
}

impl<K: Eq + Hash + Copy, V: Copy> Node<K, V> {
    fn new_node_link(key: K, value: V) -> NodeLink<K, V> {
        let node = Node { key, value, next: None, prev: None };
        Rc::new(RefCell::new(node))
    }

    fn link_prev_to_next(&mut self) {
        if let Some(prev) = &self.prev {
            match &self.next {
                None => {
                    prev.clone().as_ref().borrow_mut().next = None
                }
                Some(next) => {
                    prev.clone().as_ref().borrow_mut().next = Some(next.clone());
                }
            }
        }
    }

    fn link_next_to_prev(&mut self) {
        if let Some(next) = &self.next {
            match &self.prev {
                None => {
                    next.clone().as_ref().borrow_mut().prev = None
                }
                Some(next) => {
                    next.clone().as_ref().borrow_mut().prev = Some(next.clone());
                }
            }
        }
    }

    fn prev(&self) -> Option<NodeLink<K, V>> {
        match &self.prev {
            Some(prev_node) => {
                Some(prev_node.clone())
            }
            None => {
                None
            }
        }
    }

    fn next(&self) -> Option<NodeLink<K, V>> {
        match &self.next {
            Some(next_node) => {
                Some(next_node.clone())
            }
            None => {
                None
            }
        }
    }
}

#[allow(dead_code)]
struct Queue<K: Eq + Hash + Copy, V: Copy> {
    head: Option<NodeLink<K, V>>,
    tail: Option<NodeLink<K, V>>,
}

impl<K: Eq + Hash + Copy, V: Copy> Queue<K, V> {
    fn new() -> Self {
        Queue { head: None, tail: None }
    }

    fn push_last(&mut self, node: NodeLink<K, V>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
            return;
        }
        if let Some(cur_tail) = &self.tail {
            cur_tail.clone().as_ref().borrow_mut().next = Some(node.clone());
            node.as_ref().borrow_mut().prev = Some(cur_tail.clone());

            self.tail = Some(node);
        }
    }

    fn remove(&mut self, node: NodeLink<K, V>) {
        node.as_ref().borrow_mut().link_prev_to_next();
        node.as_ref().borrow_mut().link_next_to_prev();

        if let Some(head) = &self.head {
            if head.clone().as_ref().borrow().key == node.clone().as_ref().borrow().key {
                self.head = node.clone().as_ref().borrow().next();
            }
        }

        if let Some(tail) = &self.tail {
            if tail.clone().as_ref().borrow().key == node.clone().as_ref().borrow().key {
                self.tail = node.as_ref().borrow().prev();
            }
        }
    }

    fn pop_first(&mut self) -> NodeLink<K, V> {
        let first = self.head.as_ref().unwrap().clone();
        self.remove(first.clone());
        first
    }
}

pub struct CacheLru<K: Eq + Hash + Copy, V: Copy> {
    cache: HashMap<K, NodeLink<K, V>>,
    queue: Queue<K, V>,
    capacity: usize,
}

impl<K: Eq + Hash + Copy, V: Copy> CacheLru<K, V> {
    pub fn new(capacity: usize) -> Self {
        CacheLru { cache: HashMap::new(), queue: Queue::new(), capacity }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, key: K, value: V) {
        if let Entry::Occupied(existing_entry) = self.cache.entry(key) {
            self.queue.remove(existing_entry.get().clone());
            existing_entry.remove();
        }
        let new_node = Node::new_node_link(key, value);
        self.queue.push_last(new_node.clone());
        self.cache.insert(key, new_node);

        if self.cache.len() > self.capacity {
            let first_node = self.queue.pop_first();
            self.cache.remove(&first_node.borrow().key);
        }
    }

    #[allow(dead_code)]
    pub fn get(&mut self, key: K) -> Option<V> {
        match self.cache.entry(key) {
            Entry::Occupied(entry) => {
                let node = entry.get();
                self.queue.remove(node.clone());
                self.queue.push_last(node.clone());

                Some(node.borrow().value)
            }
            Entry::Vacant(_) => {
                None
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mut cache: CacheLru<i32, &str> = CacheLru::new(2);
        cache.set(1, "one");
        cache.set(2, "two");

        assert_eq!(cache.get(1), Some("one"));

        cache.set(3, "three");
        assert_eq!(cache.get(2), None);

        cache.set(4,"four");
        assert_eq!(cache.get(1), None);

        assert_eq!(cache.get(3), Some("three"));
        assert_eq!(cache.get(4), Some("four"));
    }

    #[test]
    fn save_value() {
        let mut cache: CacheLru<i32, &str> = CacheLru::new(2);
        assert_eq!(cache.get(0), None);

        cache.set(0,"0");
        assert_eq!(cache.get(0), Some("0"));

        cache.set(0,"zero");
        assert_eq!(cache.get(0), Some("zero"));
    }
}
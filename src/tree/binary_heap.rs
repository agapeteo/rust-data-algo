use std::collections::VecDeque;

pub struct BinaryHeap<'a, T: Ord> {
    vec: VecDeque<&'a T>
}

impl<'a, T: Ord> BinaryHeap<'a, T> {
    pub fn new() -> Self {
        BinaryHeap { vec: VecDeque::new() }
    }

    pub fn peek_min(&self) -> Option<&T> {
        self.vec.get(0).copied()
    }

    pub fn push(&mut self, element: &'a T) {
        self.vec.push_back(element);
        self.sift_up(self.vec.len() - 1);
    }

    fn sift_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }
        let parent_idx = parent_idx(idx);
        if self.vec[parent_idx] > self.vec[idx] {
            self.vec.swap(parent_idx, idx);
            self.sift_up(parent_idx);
        }
    }

    pub fn pop_min(&mut self) -> Option<&T> {
        let result = self.vec.pop_front();

        if self.vec.len() > 1 {
            let last = self.vec.pop_back().unwrap();
            self.vec.push_front(last);
            self.sift_down(0);
        }
        result
    }

    fn sift_down(&mut self, idx: usize) {
        if idx >= self.vec.len() / 2 {
            return;
        }

        let left_child_idx = left_child_idx(idx);
        let right_child_idx = right_child_idx(idx);

        let smaller_child_idx =
            if right_child_idx < self.vec.len() && self.vec[right_child_idx] < self.vec[left_child_idx] {
                right_child_idx
            } else {
                left_child_idx
            };

        if self.vec[smaller_child_idx] < self.vec[idx] {
            self.vec.swap(idx, smaller_child_idx);
            self.sift_down(smaller_child_idx);
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

fn parent_idx(idx: usize) -> usize {
    (idx - 1) / 2
}

fn left_child_idx(idx: usize) -> usize {
    idx * 2 + 1
}

fn right_child_idx(idx: usize) -> usize {
    idx * 2 + 2
}

#[test]
fn test() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    assert_eq!(heap.is_empty(), true);
    assert_eq!(heap.peek_min(), None);

    heap.push(&-1);
    assert_eq!(heap.len(), 1);

    assert_eq!(heap.peek_min(), Some(&-1));
    assert_eq!(heap.pop_min(), Some(&-1));
    assert_eq!(heap.is_empty(), true);

    heap.push(&-99);
    heap.push(&12);
    heap.push(&5);
    heap.push(&1);
    heap.push(&4);
    heap.push(&0);
    heap.push(&-9);

    assert_eq!(heap.pop_min(), Some(&-99));
    assert_eq!(heap.pop_min(), Some(&-9));

    heap.push(&10);

    let mut actual = vec![];

    while let Some(e) = heap.pop_min() {
        actual.push(e.clone());
    }

    assert_eq!(actual, [0, 1, 4, 5, 10, 12]);
}
pub struct BinaryTree<T: PartialOrd + Clone> {
    root: Option<Box<Node<T>>>
}

impl<T: PartialOrd + Clone> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn add(&mut self, value: T) {
        let new_node = Node::new_boxed(value);
        match self.root {
            None => {
                self.root.replace(new_node);
            }
            Some(ref mut root) => {
                root.add(new_node);
            }
        }
    }

    pub fn contains(&self, item: T) -> bool {
        match self.root {
            None => { false }
            Some(ref node) => {
                node.contains(item)
            }
        }
    }

    pub fn depth(&self) -> usize {
        let mut size = 0_usize;

        if let Some(root_node) = &self.root {
            size = root_node.depth(size);
        }
        size
    }

    pub fn collect_values(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        if let Some(root_node) = &self.root {
            root_node.collect_values(&mut vec);
        }
        vec
    }
}

impl<T: PartialOrd + Clone> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree::new()
    }
}

struct Node<T: PartialOrd + Clone> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + Clone> Node<T> {
    fn new_boxed(value: T) -> Box<Node<T>> {
        Box::new(Node { value, left: None, right: None })
    }

    fn add(&mut self, other_node: Box<Node<T>>) {
        if other_node.value > self.value {
            match self.right {
                None => {
                    self.right.replace(other_node);
                }
                Some(ref mut right_node) => {
                    right_node.add(other_node);
                }
            }
        } else { // add to left branch
            match self.left {
                None => {
                    self.left.replace(other_node);
                }
                Some(ref mut left_node) => {
                    left_node.add(other_node);
                }
            }
        }
    }

    fn contains(&self, item: T) -> bool {
        if self.value == item {
            return true;
        }
        if item > self.value {
            if let Some(right_node) = &self.right {
                return right_node.contains(item);
            }
        } else { // search in left branch
            if let Some(left_node) = &self.left {
                return left_node.contains(item);
            }
        }
        false
    }

    fn depth(&self, cur_size: usize) -> usize {
        let mut left_depth = 0_usize;
        if let Some(left_node) = &self.left {
            left_depth = left_node.depth(cur_size + 1);
        }

        let mut right_depth = 0_usize;
        if let Some(right_node) = &self.right {
            right_depth = right_node.depth(cur_size + 1);
        }
        if left_depth == 0 && right_depth == 0 {
            cur_size
        } else {
            std::cmp::max(left_depth, right_depth)
        }
    }

    fn collect_values(&self, vec: &mut Vec<T>) {
        if let Some(left_node) = &self.left {
            left_node.collect_values(vec);
        }
        let value = &self.value;
        vec.push(value.to_owned());

        if let Some(right_node) = &self.right {
            right_node.collect_values(vec);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_values() {
        let mut tree = BinaryTree::new();
        tree.add(6);
        tree.add(3);
        tree.add(8);
        tree.add(1);
        tree.add(2);
        tree.add(7);
        tree.add(10);
        tree.add(4);
        tree.add(5);
        tree.add(9);

        let values = tree.collect_values();
        assert_eq!(values, (1..=10).collect::<Vec<i32>>());
    }

    #[test]
    fn contains() {
        let mut tree = BinaryTree::new();
        tree.add(6);
        tree.add(3);
        tree.add(8);

        assert_eq!(tree.contains(6), true);
        assert_eq!(tree.contains(3), true);
        assert_eq!(tree.contains(8), true);

        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(1), false);
        assert_eq!(tree.contains(7), false);

        tree.add(5);
        tree.add(1);
        tree.add(7);

        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(1), true);
        assert_eq!(tree.contains(7), true);
    }

    #[test]
    fn depth() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.depth(), 0);

        tree.add(6);
        assert_eq!(tree.depth(), 0);

        tree.add(3);
        assert_eq!(tree.depth(), 1);

        tree.add(8);
        assert_eq!(tree.depth(), 1);

        tree.add(2);
        assert_eq!(tree.depth(), 2);

        tree.add(10);
        assert_eq!(tree.depth(), 2);

        tree.add(11);
        assert_eq!(tree.depth(), 3);
    }

    #[test]
    fn depth_right_unbalanced() {
        let mut tree = BinaryTree::new();
        const N: usize = 100;

        for i in 0..=N {
            tree.add(i);
        }
        assert_eq!(tree.depth(), N);
    }

    #[test]
    fn depth_left_unbalanced() {
        let mut tree = BinaryTree::new();
        const N: usize = 100;

        for i in (0..=N).rev() {
            tree.add(i);
        }
        assert_eq!(tree.depth(), N);
    }
}


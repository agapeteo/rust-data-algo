use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

// expecting all edges references in the same lifetime scope
#[allow(dead_code)]
pub struct Graph<'a, T: Eq + Hash> {
    edges: HashMap<&'a T, HashSet<&'a T>>
}

#[allow(dead_code)]
impl<'a, T: Eq + Hash> Graph<'a, T> {
    pub fn new() -> Self {
        let edges: HashMap<&'a T, HashSet<&'a T>> = HashMap::new();
        Graph { edges }
    }

    pub fn add_both_edges(&mut self, from: &'a T, to: &'a T) {
        self.add_edge(from, to);
        self.add_edge(to, from);
    }

    pub fn add_edge(&mut self, from: &'a T, to: &'a T) {
        let connections_maybe = self.edges.get_mut(&from);
        match connections_maybe {
            None => {
                let mut new_connections: HashSet<&'a T> = HashSet::new();
                new_connections.insert(to);
                self.edges.insert(from, new_connections);
            }
            Some(connections) => {
                connections.insert(to);
            }
        }
    }

    pub fn collect_elements_dfs(&self) -> Vec<&'a T> {
        let mut visited: HashSet<&'a T> = HashSet::new();
        let mut elements: Vec<&'a T> = Vec::new();

        for k in self.edges.keys() {
            self.collect_dfs(k, &mut visited, &mut elements);
        }

        elements
    }

    pub fn collect_dfs(&self, elem: &'a T, visited: &mut HashSet<&'a T>, elements: &mut Vec<&'a T>) {
        if visited.contains(&elem) {
            return;
        }
        elements.push(elem);
        visited.insert(elem);

        let connection_maybe = self.edges.get(elem);
        if let Some(connections) = connection_maybe {
            for e in connections {
                self.collect_dfs(e, visited, elements);
            }
        }
    }

    pub fn collect_elements_bfs(&self) -> Vec<&T> {
        let mut elements: Vec<&'a T> = Vec::new();
        let mut visited: HashSet<&'a T> = HashSet::new();

        for elem in self.edges.keys() {
            let mut queue: VecDeque<&T> = VecDeque::new();
            queue.push_front(elem);
            while let Some(cur_element) = queue.pop_front() {
                if visited.contains(cur_element) {
                    continue;
                }
                elements.push(cur_element);
                visited.insert(cur_element);
                if let Some(connections) = self.edges.get(cur_element) {
                    for other_element in connections {
                        queue.push_back(other_element);
                    }
                }
            }
        }
        elements
    }

    pub fn connected(&self, from: &'a T, to: &'a T) -> bool {
        let mut visited: HashSet<&T> = HashSet::new();
        let mut queue: VecDeque<&T> = VecDeque::new();

        queue.push_front(from);

        while let Some(cur_element) = queue.pop_front() {
            if visited.contains(cur_element) {
                continue;
            }

            if *to == *cur_element {
                return true;
            }

            visited.insert(cur_element);

            if let Some(connections) = self.edges.get(cur_element) {
                for other_element in connections {
                    queue.push_back(other_element);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_dfs() {
        let mut graph: Graph<&str> = Graph::new();
        let alex = "Alex";
        let bob = "Bob";
        let michael = "Michael";
        let john = "John";
        let sara = "Sara";

        graph.add_both_edges(&alex, &bob);
        graph.add_edge(&bob, &michael);
        graph.add_edge(&bob, &john);
        graph.add_edge(&john, &sara);
        graph.add_edge(&sara, &john);

        let mut elements = graph.collect_elements_dfs();
        elements.sort();

        assert_eq!(elements, vec![&alex, &bob, &john, &michael, &sara]);
    }

    #[test]
    fn collect_bfs() {
        let mut graph: Graph<&str> = Graph::new();
        let alex = "Alex";
        let bob = "Bob";
        let michael = "Michael";
        let john = "John";
        let sara = "Sara";

        graph.add_both_edges(&alex, &bob);
        graph.add_edge(&bob, &michael);
        graph.add_edge(&bob, &john);
        graph.add_edge(&john, &sara);
        graph.add_edge(&sara, &john);

        let mut elements = graph.collect_elements_bfs();
        elements.sort();

        assert_eq!(elements, vec![&alex, &bob, &john, &michael, &sara]);
    }

    #[test]
    fn check_connected() {
        let mut graph: Graph<&str> = Graph::new();
        let alex = "Alex";
        let bob = "Bob";
        let michael = "Michael";
        let john = "John";
        let sara = "Sara";

        assert_eq!(graph.connected(&alex, &bob), false);

        graph.add_both_edges(&alex, &bob);


        assert_eq!(graph.connected(&alex, &bob), true);

        graph.add_edge(&bob, &michael);
        assert_eq!(graph.connected(&alex, &michael), true);

        graph.add_both_edges(&john, &sara);
        assert_eq!(graph.connected(&john, &sara), true);
        assert_eq!(graph.connected(&alex, &sara), false);

        graph.add_edge(&michael, &john);
        assert_eq!(graph.connected(&alex, &sara), true);
    }
}
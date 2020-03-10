use std::cell::RefCell;
use std::rc::Rc;

// from: https://ricardomartins.cc/2016/06/08/interior-mutability

// A graph can be represented in several ways. For the sake of illustrating how
// interior mutability works in practice, let's go with the simplest
// representation: a list of nodes.
// Each node has an inner value and a list of adjacent nodes it is connected to
// (through a directed edge).
// That list of adjacent nodes cannot be the exclusive owner of those nodes, or
// else each node would have at most one edge to another node and the graph
// couldn't also own these nodes.
// We need to wrap Node with a reference-counted box, such as Rc or Arc. We'll
// go with Rc, because this is a toy example.
// However, Rc<T> and Arc<T> enforce memory safety by only giving out shared
// (i.e., immutable) references to the wrapped object, and we need mutability to
// be able to connect nodes together.
// The solution for this problem is wrapping Node in either Cell or RefCell, to
// restore mutability. We're going to use RefCell because Node<T> doesn't
// implement Copy (we don't want to have independent copies of nodes!).

// Represents a reference to a node.
// This makes the code less repetitive to write and easier to read.
type NodeRef<T> = Rc<RefCell<_Node<T>>>;

// The private representation of a node.
struct _Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}

// The public representation of a node, with some syntactic sugar.
struct Node<T>(NodeRef<T>);

impl<T> Node<T> {
    // Creates a new node with no edges.
    fn new(inner: T) -> Node<T> {
        let node = _Node { inner_value: inner, adjacent: vec![] };
        Node(Rc::new(RefCell::new(node)))
    }

    // Adds a directed edge from this node to other node.
    fn add_adjacent(&self, other: &Node<T>) {
        (self.0.borrow_mut()).adjacent.push(other.0.clone());
    }
}

struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    fn with_nodes(nodes: Vec<Node<T>>) -> Self {
        Graph { nodes: nodes }
    }
}

#[test]
fn check() {
    // Create some nodes
    let node_1 = Node::new(1);
    let node_2 = Node::new(2);
    let node_3 = Node::new(3);

    // Connect some of the nodes (with directed edges)
    node_1.add_adjacent(&node_2);
    node_1.add_adjacent(&node_3);
    node_2.add_adjacent(&node_1);
    node_3.add_adjacent(&node_1);

    // Add nodes to graph
    let graph = Graph::with_nodes(vec![node_1, node_2, node_3]);

    // Show every node in the graph and list their neighbors
    for node in graph.nodes.iter().map(|n| n.0.borrow()) {
        let value = node.inner_value;
        let neighbours = node.adjacent.iter()
            .map(|n| n.borrow().inner_value)
            .collect::<Vec<_>>();
        println!("node ({}) is connected to: {:?}", value, neighbours);
    }
}
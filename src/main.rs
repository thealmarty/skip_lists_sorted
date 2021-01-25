// use fastrand::bool;

fn main() {
    let node1 = build_node(20, vec![1, 2, 3], 3);
    let key_node_pair1 = build_key_node_pair(1, node1);
    println!("key node pair 1 is {:?}", key_node_pair1);
}

fn build_node(val: i32, next: ListOfPointers, height: usize) -> Node {
    Node { val, next, height }
}

fn build_key_node_pair(key: KeyType, node: Node) -> KeyNodePair {
    (key, node)
}

type ValueType = i32; // the type of the value of the list.

type KeyType = usize; // the type of the key of the list.

// the list is a list of key/value pair.
type KeyNodePair = (KeyType, Node);

// a list of pointers, pointing to the node's successor at a given level i
type ListOfPointers = Vec<usize>;

// a skip list is a sentinel and a list of key/node pair
type SkipList = (ListOfPointers, Vec<KeyNodePair>);

#[derive(Debug)]
// A node containing an element and a list of pointers.
struct Node {
    val: ValueType, // the element/value
    next: ListOfPointers,
    height: usize, // the current height
}

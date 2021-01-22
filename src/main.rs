// use fastrand::bool;

fn main() {
    let node1 = build_node(20, vec![1, 2, 3], 3);

    println!("node1 is {:?}", node1);
}

fn build_node(ele: usize, next: Vec<i32>, height: i32) -> Node {
    Node { ele, next, height }
}

#[derive(Debug)]
// A node containing an element and a list of pointers.
struct Node {
    ele: usize, // the element
    // a list of pointers, pointing to the node's successor at a given level i
    next: Vec<i32>,
    height: i32, // the current height
}

use std::rc::Rc;
use fastrand::bool;

fn main() {}

fn add<T>(val: T, slist: SkipList<T>) -> SkipList<T> {
    if slist.len() == 0 { // adding to an empty list
        let mut new_node = Node<T> {
            val,
            Vec::new()
        }
        while fastrand::bool() { // the height is determined by a coin flip
            new_node.next.push(None); // the next node of val is none
            // the sentinel (of height the same as val) points to val
            slist.push(Some(new_node))
        }
    } else {
        //TODO
        Vec::new()
    }
}

// a vector of successive nodes, at a given level i.
// the successive node could be none.
type VecOfNodes<T> = Vec<Rc<Option<Node<T>>>>;

// a skip list is a vec of nodes.
type SkipList<T> = VecOfNodes<T>;

#[derive(Debug)]
// A node contains a value of type T and a vec of nodes.
struct Node<T> {
    val: T,              // the element/value
    next: VecOfNodes<T>, // a list of next nodes
}

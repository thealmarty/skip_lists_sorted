use fastrand::bool; // for coin flipping
use std::cmp::Ordering; // TODO ordering of val required.
use std::rc::Rc;

fn main() {}

fn add<T>(val: T, mut slist: SkipList<T>) -> SkipList<T> {
    if true {
    //TODO find_node<T>(val, slist) == None { // if val doesn't exist, add it
        // initialize new node with val
        let mut new_node = Node {
            val,
            next: Vec::new(),
        };
        // figure out the correct next of val to add the new node
        let next_node = find_next<T>(val, slist);
        while fastrand::bool() { // the height is determined by a coin flip
            // points to the next node of val
            while next_node.next.len() >= new_node.next.len() {
                new_node.next.push(Rc::new(next_node));
            };
            // TODO the extra height has to point to the node after
        };
        // TODO update the pointers of the previous node to point to new node
        // TODO if pred_node is nil, need to update the whole sentinel to point
        // at new val
        // update the added height of the sentinel
        let counter = 0;
        while slist.len() < new_node.next.len() - counter {
            // if val is higher then the sentinel, the height of the sentinel
            // increases to the same as the height of val, 
            // and the increased pointers point to val.
            slist.push(Rc::new(Some(new_node)));
            counter+=1;
        };
        slist
    } else { // if val exists, update val to new val
    slist //TODO    
    } 
}

// TODO
fn find_node<T>(val: T, slist: SkipList<T>) -> Option<SkipList<T>> {
    // let last_index = slist.len();
    // if val > slist
    None
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

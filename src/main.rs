// use fastrand::bool; // for coin flipping
// use std::cmp::Ordering; // TODO ordering of val required.
//                         // TODO change to atomic nodes to allow for concurrency
// use std::borrow::Borrow;

fn main() {}

#[derive(Debug)]
// A node contains a value of type T and a vec of nodes.
struct Node<T: Ord> {
    val: T,              // the element/value
    next: VecOfNodes<T>, // a list of next nodes
}

// a vector of successive nodes, at a given level i.
// the successive node could be none.
type VecOfNodes<T> = Vec<Option<Node<T>>>;

// a skip list is a vec of nodes.
type SkipList<T> = Vec<Node<T>>;

// finds if val is in the slist. If not, it returns none. Otherwise, it returns
// the node that has val equals to the input val.
fn find_node<T: Ord>(val: T, slist: SkipList<T>) -> Option<Node<T>> {
    if slist.len() == 0 {
        None
    } else {
        let mut cur_height = slist.len() - 1; // current height
                                              // moving off the sentinel
        while cur_height > 0 && slist[cur_height].val > val {
            cur_height -= 1;
        } // moved off the sentinel
        let mut cur_node = &slist[cur_height];
        while cur_height > 0 {
            match &cur_node.next[cur_height] {
                None => cur_height -= 1,
                Some(node) => {
                    if node.val > val {
                        cur_height -= 1;
                    } else {
                        cur_node = node;
                    }
                }
            }
        }
        if cur_node.val == val {
            None // fix this Some(cur_node)
        } else {
            None
        }
    }
}

// fn add<T>(val: T, mut slist: SkipList<T>) -> SkipList<T> {
//     if true {
//     //TODO find_node<T>(val, slist) == None { // if val doesn't exist, add it
//         // initialize new node with val
//         let mut new_node = Node {
//             val,
//             next: Vec::new(),
//         };
//         // figure out the correct next of val to add the new node
//         let next_node = find_next<T>(val, slist);
//         while fastrand::bool() { // the height is determined by coin flips
//             // points to the next node of val
//             while next_node.next.len() >= new_node.next.len() {
//                 new_node.next.push(Rc::new(next_node));
//             };
//             // TODO the extra height has to point to the node after
//         };
//         // TODO update the pointers of the previous node to point to new node
//         // if next_node was the first node, need to update the whole sentinel to point
//         // at new val
//         if next_node.val == slist(0) {

//         }
//         // update the added height of the sentinel
//         let mut counter = 0;
//         while slist.len() < new_node.next.len() - counter {
//             // if val is higher then the sentinel, the height of the sentinel
//             // increases to the same as the height of val,
//             // and the increased pointers point to val.
//             slist.push(Rc::new(Some(new_node)));
//             counter+=1;
//         };
//         slist
//     } else { // if val exists, update val to new val
//     slist //TODO
//     }
// }

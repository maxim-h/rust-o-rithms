#![allow(dead_code)]
#![allow(unused_imports)]

pub mod character_queue;
pub mod binary_tree;

use std::io::stdin;
//use std::collections::HashMap;
use character_queue::PriorityQueue;
use character_queue::FromString;
use binary_tree::Node;


fn main() {
    /* TEST PriorityQueue
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let mut q: PriorityQueue<char> = PriorityQueue::from_string(&s);

    println!("{:?}", q.extract_min());
    */

    let n1= Node::new(Option::from('f'), Option::from(3), Option::from(None), Option::from(None));
    let n2= Node::new(Option::from('c'), Option::from(5), Option::from(None), Option::from(None));
    let n3= Node::new(Option::from(None), Option::from(None), Some(n1), Some(n2));
    print!("{:?}", n3)
}

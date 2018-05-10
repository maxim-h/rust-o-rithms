#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]


pub mod character_queue;
pub mod binary_tree;

use std::io::stdin;
//use std::collections::HashMap;
use character_queue::PriorityQueue;
use character_queue::ObjectInQueue;
use character_queue::FromString;
use binary_tree::Node;


/*
fn char_code(n: Box<Node>, s: char, prefix: String) -> String{
    //when used n should be the root of a Haffman tree
    let n_char: char = match n.ch {
        Some(c) => {},
        None => "c",
    };

    if n_char == s {}
}
*/





fn main() {
    // TEST PriorityQueue
/*    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let mut q: PriorityQueue<char> = PriorityQueue::from_string(&s);

    println!("{:?}", q.extract_min());*/
    /* TEST binary_tree
    let n1= Node::new(Option::from('f'), Option::from(3), Option::from(None), Option::from(None));
    let n2= Node::new(Option::from('c'), Option::from(5), Option::from(None), Option::from(None));
    let n3= Node::new(Option::from(None), Option::from(None), Some(n1), Some(n2));
    print!("{:?}", n3)
    */

    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let mut q: PriorityQueue<Box<Node>> = PriorityQueue::from_string(&s);


    for k in q.len()+1..2*q.len() {
        let i: ObjectInQueue<Box<Node>> = q.extract_min();
        let (i, pr_i) = (i.obj, i.priority);
        let j: ObjectInQueue<Box<Node>> = q.extract_min();
        let (j, pr_j) = (j.obj, j.priority);

        q.insert(Node::new(Option::from(None), Option::from(None), Some(i), Some(j)), pr_i + pr_j);
    }
    // After this cycle root of my binary_tree will be the only element in the PriorityQueue
    println!("{:?}", q.extract_min())


}

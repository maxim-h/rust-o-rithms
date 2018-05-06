pub mod character_queue;

use std::io::stdin;
//use std::collections::HashMap;
use character_queue::PriorityQueue;
use character_queue::FromString;



fn main() {
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let mut q: PriorityQueue<char> = PriorityQueue::from_string(&s);


    /*
    queue.insert(1, 12);
    queue.insert(2, 11);
    queue.insert(3, 10);
    queue.insert(4, 9);
    queue.insert(5, 8);
    queue.insert(6, 7);

    println!("{:?}", queue.extract_min());
    */
    println!("{:?}", q.extract_min());
}

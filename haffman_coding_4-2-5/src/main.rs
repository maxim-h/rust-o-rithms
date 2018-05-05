use std::io::stdin;
use std::collections::HashMap;


/*
struct Node {
    parent: Option<&Node>,
    child1: &Node,
    child2: &Node,
}


struct BinaryTree {
    nodes: Vec<Node>
}


impl Node {
    fn new(child1: &Node, child2: &Node) -> Node {
        Node { parent: None, child1, child2 }
    }


}
*/
#[derive(Debug)]
struct ObjectInQueue<T> {
    obj: T,
    priority: u32,
}

impl<T> ObjectInQueue<T> {
    fn new(obj: T, priority: u32) -> ObjectInQueue<T>{
        ObjectInQueue { obj, priority}
    }
}

#[derive(Debug)]
struct PriorityQueue<T> {
    q: Vec<ObjectInQueue<T>>,
}


impl<T> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { q: Vec::new()}
    }

    fn insert<'a>(&'a mut self, value: T, priority: u32) {
        self.q.push(ObjectInQueue::new(value, priority))
    }

    fn extract_min(&mut self) -> ObjectInQueue<T> {
        let ind: usize = self.q.iter().enumerate().min_by_key(|&x| x.1.priority).unwrap().0;
        self.q.remove(ind)
    }
}

pub fn char_frequency(s: &String) -> HashMap<char, u32> {
    let mut frequency: HashMap<char, u32> = HashMap::new();
    for ch in s.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }
    frequency
}


fn main() {
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let f = char_frequency(&s);

    let mut queue: PriorityQueue<char> = PriorityQueue::new();

    for (ch, fr) in f {
        queue.insert(ch, fr);
    }


    /*
    queue.insert(1, 12);
    queue.insert(2, 11);
    queue.insert(3, 10);
    queue.insert(4, 9);
    queue.insert(5, 8);
    queue.insert(6, 7);

    println!("{:?}", queue.extract_min());
    */
    println!("{:?}", queue.extract_min());
}

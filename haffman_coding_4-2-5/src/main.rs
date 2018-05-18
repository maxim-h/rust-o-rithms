extern crate itertools;

use std::io::stdin;
use itertools::Itertools;
use std::collections::HashMap;


pub mod character_queue;
pub mod binary_tree;

use character_queue::PriorityQueue;
use character_queue::ObjectInQueue;
use character_queue::FromString;
use binary_tree::Node;
use binary_tree::encode_char;


fn main() {
    //read string from stdin
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let mut q: PriorityQueue<Box<Node>> = PriorityQueue::from_string(&s);

    for _ in q.len() + 1..2 * q.len() {
        let i: ObjectInQueue<Box<Node>> = q.extract_min();
        let (i, pr_i) = (i.obj, i.priority);
        let j: ObjectInQueue<Box<Node>> = q.extract_min();
        let (j, pr_j) = (j.obj, j.priority);

        q.insert(Node::new(Option::from(None), Option::from(None), Some(i),
                           Some(j)), pr_i + pr_j);
    }
    // After this cycle root of my binary_tree will be the only element in the PriorityQueue

    //get a pointer to my haffman tree root
    let haffman_tree: &Box<Node> = &q[0];

    let unique_chars: Vec<char> = s.chars().unique().collect();

    //create encoder HashMap and reserve memory for future entries
    let mut encoder: HashMap<char, String> = HashMap::new();
    encoder.reserve(unique_chars.len());

    //populate encoder
    for ch in &unique_chars {
        encoder.entry(*ch).or_insert(match encode_char(
            haffman_tree,
            *ch,
            String::new()
        ) {
            Some(code)=> code,
            None => panic!("Couldn't encode {}, you dumb fuck!", ch),
        }
        );
    }

    let mut encoded_string: String = String::new();

    for ch in s.chars() {
        encoded_string.push_str(match encoder.get(&ch) {
            Some(code) => code,
            None => panic!("Couldn't get a code for {} from the encoder", ch),
        });
    }

    println!("{} {}", unique_chars.len(), encoded_string.chars().count());
    for (key, value) in encoder {
        println!("{}: {}", key, value)
    }
    println!("{:?}", encoded_string);
}

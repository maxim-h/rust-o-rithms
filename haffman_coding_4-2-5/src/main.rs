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


#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate rayon;
    use self::rand::distributions::{IndependentSample, Range};
    use self::rand::Rng;
    use self::rayon::prelude::*;
    use super::*;


    fn random_string(len: usize) -> String {
        let rstr: String = rand::thread_rng()
            .gen_iter::<char>()
            .take(len)
            .collect();
        rstr
    }

    fn decode_encoded() -> i32 {
        let mut rng = rand::thread_rng();
        let range: Range<u32> = Range::new(1, 10000);
        let mut test_string = random_string(range.ind_sample(&mut rng) as usize);

        let encoder = make_encoder(&test_string,
                                   haffman_tree(&test_string));

        let decoder = make_decoder(&encoder);

        assert_eq!(test_string, decode_string(encode_string(&test_string, &encoder),
                                              decoder));
        1
    }

    #[test]
    fn decode_encoded_test() {
        let size = [0; 100]; //hacky
        let sum: i32 = size.par_iter()
                           .map(| _i | decode_encoded())
                           .sum();
        assert_eq!(sum, 100);
    }
}


fn haffman_tree(s: &String) -> Box<Node> {
    let mut q: PriorityQueue<Box<Node>> = PriorityQueue::from_string(s);

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
    q[0].clone()
}


fn make_encoder(s: &String, haff_tree: Box<Node>) -> HashMap<char, String> {
    let unique_chars: Vec<char> = s.chars().unique().collect();

    //create encoder HashMap and reserve memory for future entries
    let mut encoder: HashMap<char, String> = HashMap::new();
    encoder.reserve(unique_chars.len());

    //populate encoder
    for ch in &unique_chars {
        encoder.entry(*ch).or_insert(match encode_char(
            &haff_tree,
            *ch,
            String::new()
        ) {
            Some(code)=> code,
            None => panic!("Couldn't encode {}, you dumb fuck!", ch),
        }
        );
    }
    encoder
}


fn encode_string(s: &String, encoder: &HashMap<char, String>) -> String {
    let mut encoded_string: String = String::new();
    for ch in s.chars() {
        encoded_string.push_str(match encoder.get(&ch) {
            Some(code) => code,
            None => panic!("Couldn't get a code for {} from the encoder", ch),
        });
    }
    encoded_string
}

fn make_decoder(encoder: &HashMap<char, String>) -> HashMap<String, char> {
    let mut decoder: HashMap<String, char> = HashMap::new();

    for (key, value) in encoder.clone() {
        decoder.entry(value).or_insert(key);
    }
    decoder
}


fn decode_string(encoded: String, decoder: HashMap<String, char>) -> String {
    let mut decoded: String = String::new();
    let mut prefix: String = String::new();

    for ch in encoded.chars() {
        prefix.push(ch);
        if let Some(char) = decoder.get(&prefix) {
            decoded.push(*char);
            prefix.clear();
        }
    }
    decoded
}


fn main() {
    //read string from stdin
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Didn't read, lol");
    let s: String = s.trim().parse().expect("Couldn't parse, lol");

    let encoder = make_encoder(&s, haffman_tree(&s));

    let encoded_string: String = encode_string(&s, &encoder);

    let unique_chars: Vec<char> = s.chars().unique().collect();


    println!("{} {}", unique_chars.len(), encoded_string.chars().count());
    for (key, value) in &encoder {
        println!("{}: {}", key, value)
    }
    println!("{:?}", encoded_string);
    println!("{:?}", decode_string(encoded_string, make_decoder(&encoder)));
}

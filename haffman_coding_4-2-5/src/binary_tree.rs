use std::boxed::Box;
use std::borrow::Borrow;
/// Simple binary tree implementation
/// As of right now only Node is implemented.
/// Struct character_queue::PriorityQueue serves as the tree itself
#[derive(Debug, Clone)]
pub struct Node {
    pub ch: Option<char>,
    pub freq: u32,
    pub l_0: Option<Box<Node>>,
    pub r_1: Option<Box<Node>>,
}


impl Node {

    fn get_freq<'a>(&'a self) -> &'a u32 {
        &self.freq
    }

    pub fn new(c: Option<char>, f: Option<u32>, l: Option<Box<Node>>, r: Option<Box<Node>> ) -> Box<Node> {
        let mut leaf = false;
        let mut left= Box::new(Node {
            ch: None,
            freq: 0,
            l_0: None,
            r_1: None,
        });
        let mut right= Box::new(Node {
            ch: None,
            freq: 0,
            l_0: None,
            r_1: None,
        });

        match l {
            Some(l) => {left = l;},
            None => {leaf  = true;},
        };

        match r {
            Some(r) => {right = r;},
            None => {leaf = true;},
        };

        if !leaf {
            let sum_freq = right.get_freq() + left.get_freq();

            return Box::new(Node {
                        ch: None,
                        freq: sum_freq,
                        l_0: Option::from(left),
                        r_1: Option::from(right),
                        })
        } else {
            return Box::new(Node {
                        ch: c,
                        freq: f.unwrap(),
                        l_0: None,
                        r_1: None,
                        })
        }
    }
}


pub fn encode_char(n: &Node, c: &char, prefix: String) -> Option<String> {
    if let Some(x) = n.ch {
        if x == *c {
            return Some(prefix);
        } else {
            return None
        }
    } else {
        let pr = format!("{}0", prefix);
        match n.l_0 {
            Some(ref nl) => {
                match encode_char(nl, c, pr) {
                    None => {},
                    Some(p) => { return Some(p) },
                }
            },
            None => {panic!("Node has no left child and is not a leaf")},
        }
        let pr = format!("{}1", prefix);
        match n.r_1{
            Some(ref nr) => {
                match encode_char(nr, c, pr) {
                    None => { return None },
                    Some(p) => { return Some(p) },
                }
            },
            None => {panic!("Node has no right child and is not a leaf")},
        }
    }
}




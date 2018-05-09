use std::boxed::Box;
/// Simple binary tree implementation
///TODO: implement traits:
///                       .remove
///
///
#[derive(Debug)]
pub struct Node {
    ch: Option<char>,
    freq: u32,
    l_0: Option<Box<Node>>,
    r_1: Option<Box<Node>>,
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



/// Simple binary tree implementation
///TODO: implement traits:
///                       .remove
///




struct Node {
    freq: u32,
    ch: Option<char>,
    l_0: Option<Box<Node>>,
    r_1: Option<Box<Node>>,
}


impl Node {
    fn new(f: Option<u32>, c: Option<char>, l: Option<Box<Node>>, r: Option<Box<Node>> ) -> Node {
        Node {
              freq: match f {
                  Some(f) => f,
                  None(_) => l.freq + r.freq,
              },
              ch: match c {
                  Some(c) => c,
                  None(_) => None,
              },
              l_0: match l {
                  Some(l) => l,
                  None(_) => None,
              },
              r_1: match r {
                  Some(r) => r,
                  None(_) => None,
              },
             }
    }

}
/*

struct BinaryTree {
    nodes: Vec<Node>
}




*/

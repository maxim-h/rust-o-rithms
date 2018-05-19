use std::collections::HashMap;
use binary_tree::Node;
use std::ops::Index;

#[derive(Debug)]
pub struct ObjectInQueue<T> {
    pub obj: T,
    pub priority: u32,
}

impl<T> ObjectInQueue<T> {
    fn new(obj: T, priority: u32) -> ObjectInQueue<T> {
        ObjectInQueue { obj, priority }
    }
}

#[derive(Debug)]
pub struct PriorityQueue<T> {
    q: Vec<ObjectInQueue<T>>,
}

impl<T> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { q: Vec::new() }
    }

    pub fn insert<'a>(&'a mut self, value: T, priority: u32) {
        self.q.push(ObjectInQueue::new(value, priority))
    }

    pub fn extract_min(&mut self) -> ObjectInQueue<T> {
        let ind: usize = self.q
            .iter()
            .enumerate()
            .min_by_key(|&x| x.1.priority)
            .expect("Using extract_min on an empty PriorityQueue")
            .0;
        self.q.remove(ind)
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }
}

// To index PriorityQueue directly
// Note that it returns the `obj: T` itself instead of ObjectInQueue<T>
impl<T> Index<usize> for PriorityQueue<T> {
    type Output = T;

    fn index(&self, ind: usize) -> &T {
        &self.q[ind].obj
    }
}

pub trait FromString<T> {
    fn from_string<'a>(s: &'a String) -> PriorityQueue<T>;
}

fn char_frequency(s: &String) -> HashMap<char, u32> {
    let mut frequency: HashMap<char, u32> = HashMap::new();
    for ch in s.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }
    frequency
}

impl FromString<Box<Node>> for PriorityQueue<Box<Node>> {
    fn from_string<'a>(s: &'a String) -> PriorityQueue<Box<Node>> {
        let mut queue: PriorityQueue<Box<Node>> = PriorityQueue::new();
        let f = char_frequency(s);
        for (ch, fr) in f {
            queue.insert(
                Node::new(
                    Option::from(ch),
                    Option::from(fr),
                    Option::from(None),
                    Option::from(None),
                ),
                fr,
            )
        }
        queue
    }
}

use std::collections::HashMap;


#[derive(Debug)]
pub struct ObjectInQueue<T> {
    obj: T,
    priority: u32,
}


impl<T> ObjectInQueue<T> {
    fn new(obj: T, priority: u32) -> ObjectInQueue<T>{
        ObjectInQueue { obj, priority}
    }
}


#[derive(Debug)]
pub struct PriorityQueue<T> {
    q: Vec<ObjectInQueue<T>>,
}


impl<T> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { q: Vec::new()}
    }

    pub fn insert<'a>(&'a mut self, value: T, priority: u32) {
        self.q.push(ObjectInQueue::new(value, priority))
    }

    pub fn extract_min(&mut self) -> ObjectInQueue<T> {
        let ind: usize = self.q.iter().enumerate().min_by_key(|&x| x.1.priority).unwrap().0;
        self.q.remove(ind)
    }
}


pub trait FromString {
    fn from_string<'a>(s: &'a String) -> PriorityQueue<char>;
}


fn char_frequency(s: &String) -> HashMap<char, u32> {
    let mut frequency: HashMap<char, u32> = HashMap::new();
    for ch in s.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }
    frequency
}


impl FromString for PriorityQueue<char> {
    fn from_string<'a>(s: &'a String) -> PriorityQueue<char> {
        let mut queue: PriorityQueue<char> = PriorityQueue::new();
        let f = char_frequency(s);
        for (ch, fr) in f {
            queue.insert(ch, fr);
        }
        queue
    }
}

    pub fn from_string<'a>(s: &'a String) -> PriorityQueue<char> {
        let mut queue: PriorityQueue<char> = PriorityQueue::new();
        let f = char_frequency(s);
        for (ch, fr) in f {
            queue.insert(ch, fr);
        }
        queue
    }


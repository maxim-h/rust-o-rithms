extern crate rand;
extern crate itertools;

use std::io::stdin;
use rand::distributions::{IndependentSample, Range}; //for testing
use itertools::Itertools; //to display Vec since i can't impl Display to it


pub fn make_slots(n: usize) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();
    let range: Range<u32> = Range::new(0, 1000_000_000);
    let mut res: Vec<(u32, u32)> = Vec::new();

    for _ in 0..n {
        let (x, y) = (range.ind_sample(&mut rng), range.ind_sample(&mut rng));
        if x > y {
            res.push((y, x))
        } else {
            res.push((x, y))
        };
    }
    return res;
}


macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lot_gen() {
        let slots: Vec<(u32, u32)> = make_slots(10000);
        for i in &slots {
            assert!(i.0 <= i.1);
        }
    }
    #[test]
    fn test_sort() {
        let mut slots: Vec<(u32, u32)> = make_slots(10000);
        slots.sort_unstable_by_key(|k| k.0);
        for i in 0..slots.len() - 1 {
            /*let (x, y) = (&slots[i].0, &slots[i+1].0);*/
            assert!(&slots[i].0 <= &slots[i + 1].0);
        }
    }
}

fn main() {
   
    let mut n: String = String::new();

    stdin().read_line(&mut n).expect("Didn't read, lol");
    let n: usize = n.trim().parse().expect("Couldn't parse, lol");
    
    /*
    let mut slots: Vec<(u32, u32)> = Vec::new();

    for _ in 0..n {
        slots.push(parse_line!(u32, u32));
    }
    */
    let mut slots: Vec<(u32, u32)> = make_slots(n);
    slots.sort_unstable_by_key(|k| k.1);
    // sort slots by end coordinates

    println!("{:?}", slots);

    let mut dots: Vec<&u32> = Vec::new();
    let mut skip: bool = false;
    let mut max: u32 = slots[0].1;
    for i in 0..slots.len() {
        if !skip {
            dots.push(&slots[i].1);
            max = slots[i].1
        };
        if i < slots.len() - 1 {
            if max > slots[i + 1].0 {
                skip = true;
            } else {
                skip = false;
            };
        }
    };
    println!("{:?}", dots.len());
    println!("{}", dots.iter().join(" "));
}

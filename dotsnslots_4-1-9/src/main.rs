extern crate rand;
use std::io::stdin;
use rand::distributions::{IndependentSample, Range};

pub fn make_slots(n: usize) -> Vec<(u32, u32)>{
    let mut rng = rand::thread_rng();
    let range: Range<u32> = Range::new(0, 1000_000_000);
    let mut res: Vec<(u32, u32)> = Vec::new();

    for _ in 0..n {
        let (x, y) = (range.ind_sample(&mut rng), range.ind_sample(&mut rng));
        if x > y {
            res.push((y, x))
        } else{
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
        for i in 0..slots.len()-1 {
           /*let (x, y) = (&slots[i].0, &slots[i+1].0);*/
            assert!(&slots[i].0 <= &slots[i+1].0);
        }
    }
}


fn main() {
    /*
    */
    let mut n: String = String::new();

    stdin().read_line(&mut n)
        .expect("Didn't read, lol");
    let n: usize =  n.trim().parse()
        .expect("Couldn't parse, lol");

    let mut slots: Vec<(u32, u32)> = Vec::new();

    for _ in 0..n {
        slots.push(parse_line!(u32, u32));
    }

    //let mut slots: Vec<(u32, u32)> = make_slots(n);
    slots.sort_unstable_by_key(|k| k.0);

    let mut dots: Vec<&u32> = Vec::new();
    let mut skip: bool = false;
    let mut max: &u32 = &0;
    for i in 0..slots.len()-1 {
        if !skip{
            let p= &slots[i].0;
            dots.push(p);
        };
        if i < slots.len()-1 {
            if &slots[i].1 > &slots[i+1].0 {
                skip = true;
            } else if &slots[i+1].1 > &max {
                skip = false;
            } else {
                skip = true;
                max = &slots[i+1].1
            };
        }

    }
    println!("{:?}", dots.len());
    for i in dots {
        println!("{:?}", i);
    }

}

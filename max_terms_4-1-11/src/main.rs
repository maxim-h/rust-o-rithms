extern crate itertools;
extern crate rand;

use rand::distributions::{IndependentSample, Range};


use std::io::stdin;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_inputs() {
        let mut rng = rand::thread_rng();
        let range: Range<i32> = Range::new(0, 1000_000_000);
        for _ in 0..10000 {
            let n = range.ind_sample(&mut rng);
            let (_, terms) = max_terms(n);
            let sum: i32 = terms.iter().sum();
            assert_eq!(sum, n);
        }
    }


}




fn max_terms(k: i32) -> (i32, Vec<i32>){
    let mut terms: Vec<i32> = Vec::new();
    let mut high: i32 = k;
    if high == 1 {
        terms.push(high);
        return(terms.len() as i32, terms);
    };
    if high == 3 {
        terms.push(1);
        terms.push(2);
        return(terms.len() as i32, terms);
    };
    for i in 1..(k/2)+1 {
        if (high - i) > i {
            terms.push(i);
            //terms.push(high - i);
            high -= i        
        } else if (high - i) < i {
            terms.push(high);
            break
        } else {
            terms.push(high);
            break
        }
    }
    return (terms.len() as i32, terms)
}

fn main() {
    let mut n: String = String::new();

    stdin().read_line(&mut n).expect("Didn;t read, lol");
    let n: i32 = n.trim().parse().expect("Couldn't parse, lol");
    let (nterms, terms) = max_terms(n);

    println!("{:?}", nterms);
    //println!("{:?}", 1..(3/2)+1);
    println!("{}", terms.iter().join(" "));
}

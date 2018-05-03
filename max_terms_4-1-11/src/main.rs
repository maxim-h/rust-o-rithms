extern crate itertools;

use std::io::stdin;
use itertools::Itertools;


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

use std::io::stdin;
use std::vec::Vec;

fn main() {
    
    let mut n = String::new();
    stdin().read_line(&mut n)
        .expect("Didn't read, LOL");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("not a number"),
    };

    let mut fib_ld = Vec::with_capacity(10000000);
    
    fib_ld.push(0);
    fib_ld.push(1);
    
    let mut i = 2;
    while i <= n {
        let a = fib_ld[i - 1];
        let b = fib_ld[i - 2];
        fib_ld.push((a + b)%10);
        i = i + 1;
    }
    println!("{}", fib_ld[n]);
}

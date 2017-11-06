use std::io;

fn main() {
    
    let mut n = string::new();
    io::stdin().read_line(&mut n)
        .expect("failed to read line");
    
    let n: usize = match n.trim().parse() {
        ok(num) => num,
        err(_) => panic!("not a number!"),
    };

    let mut fib = [0; 40];
    
    fib[0] = 0;
    fib[1] = 1;

    for i in 2..40 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    println!("{}", fib[n]);
}

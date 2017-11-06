pub fn euclidean_gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        
        if a > b {
            return euclidean_gcd(a % b, b);
        
        } else if b > a {
            return euclidean_gcd(a, b % a);
        } else {
            return a;
        }
        
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd_big() {
        assert_eq!(euclidean_gcd(14159572, 63967072), 4)
    }
    #[test]
    fn test_gcd_small() {
        assert_eq!(euclidean_gcd(18, 35), 1)
    }

}

use std::io::stdin;
fn main() {
    
    let mut input = String::new();
    stdin().read_line(&mut input)
           .expect("Didn't read, LoL!");
    let v: Vec<&str> = input.split(" ").collect();

    let a: i32 = match v[0].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Can't parse a to numeric!"),
    };
    let b: i32 = match v[1].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Can't parse b to numeric!"),
    };

    println!("{}", euclidean_gcd(a, b));
}

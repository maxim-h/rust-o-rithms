use std::io::stdin;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn equality_of_power_and_multiplication() {
        let matrix = Matrix {c1: [1, 1], c2: [1, 0]};
        let matr = matrix.clone();
        assert_eq!(matrix.power(2), matrix.multiply(matr));
    }
}






#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix {
    c1: [u64; 2],
    c2: [u64; 2],

}

pub trait Multiply {
    fn multiply(&self, m: Matrix) -> Matrix;
    fn power(&self, p: u64) -> Matrix;
}

//impl Copy for Matrix {}

impl Multiply for Matrix {
    fn multiply(&self, m: Matrix) -> Matrix {
        Matrix {c1: [self.c1[0] * m.c1[0] + self.c2[0] * m.c1[1], self.c1[1] * m.c1[0] + self.c2[1] * m.c1[1]],
                c2: [self.c1[0] * m.c2[0] + self.c2[0] * m.c2[1], self.c1[1] * m.c2[0] + self.c2[1] * m.c2[1]]}            
    }

    fn power(&self, p: u64) -> Matrix {
        let mut matr = self.clone();
        let _self = self.clone();
        if p == 0 {
            return Matrix {c1: [1, 0], c2: [0, 1]}
        }
        
        for i in 0..(p - 1) {
            let mat = _self.clone();
            matr = matr.multiply(mat);
    }
    matr
    }
}



fn main() {

    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Didn't read, Lol!");
    let v: Vec<&str> = input.split(' ').collect();

    
    let a: u64 = match v[0].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Can't parse a to numeric"),
    };
    let b: u64 = match v[1].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Can't parse b to numeric"),
    };
    
    println!("{0} and {1}", a, b);
    
    let mut matr = Matrix {c1: [1, 1], c2: [1, 0]};
    
    // println!("{:?}", matr.power(b));
    /*
    for i in 0..b {
        println!("{:?}", matr.power(i));
    }
    */

    
    //println!("{:?}", matr);


}

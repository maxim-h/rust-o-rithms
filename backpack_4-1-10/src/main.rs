use std::io::stdin;
use std::cmp::Ordering::Equal;
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

#[derive(Debug)]
pub struct Object {
    value: f64,
    weight: f64,
    price: f64,
}

impl Object {
    pub fn new(value: f64, weight: f64) -> Object {
        Object {value: value,
                weight: weight,
                price: value / weight,
        }
    }
}

fn fill_backpack(o: &mut Vec<Object> , w: f64) -> f64 {
    o.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Equal));
    let mut current_weight: f64 = 0.0;
    let mut current_sum: f64 = 0.0;
    for i in o {
        if current_weight >= w {
            break
        }
        if w - current_weight >= i.weight {
            current_weight += i.weight;
            current_sum += i.value;
        } else {
            current_sum += i.price * (w - current_weight);
            current_weight = w;
        }
}
    return current_sum    
}

fn main() {
    //let mut n: String = String::new();

    //stdin().read_line(&mut n).expect("Didn;t read, lol");
    let n = parse_line!(u32, f64);
    let w = n.1;
    let n = n.0;
    
    let mut objects: Vec<Object> = Vec::new();

    for _ in 0..n {
        let mut obj = parse_line!(f64, f64);
        let mut obj: Object = Object::new(obj.0, obj.1);
        //let mut obj: Object = Object {value: obj.0, weight: obj.1};
        //obj.count_price();
        objects.push(obj);
    }
    //println!("{:?}", objects);
    println!("{:.4}", fill_backpack(&mut objects, w))
}

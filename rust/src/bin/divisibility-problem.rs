use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut it = buffer.split_whitespace();

    let t = it.next().unwrap().parse::<i64>().unwrap();

    for _ in 0..t {
        let a = it.next().unwrap().parse::<i64>().unwrap();
        let b = it.next().unwrap().parse::<i64>().unwrap();
        let res = (b - a % b) % b;
        println!("{}", res);
    }
}

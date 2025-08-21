use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<i64>().unwrap();

    for _ in 0..t {
        let n = it.next().unwrap().parse::<String>().unwrap();
        let (i, j) = n.split_at(1);
        let sum = i.parse::<i64>().unwrap() + j.parse::<i64>().unwrap();
        println!("{}", sum);
    }
}

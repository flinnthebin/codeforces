use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut it = buffer.split_whitespace();

    let n = it.next().unwrap().parse::<i64>().unwrap();
    let k = it.next().unwrap().parse::<i64>().unwrap();

    let odds = (n + 1) / 2;
    let res = if k <= odds { 2 * k - 1 } else { 2 * (k - odds) };
    println!("{}", res);
}

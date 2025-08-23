use std::collections::HashSet;
use std::io::{self, Read};

fn shoes(a: usize, b: usize, c: usize, d: usize) -> usize {
    let hs: HashSet<_> = [a, b, c, d].into_iter().collect();
    4 - hs.len()
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let a = it.next().unwrap().parse::<usize>().unwrap();
    let b = it.next().unwrap().parse::<usize>().unwrap();
    let c = it.next().unwrap().parse::<usize>().unwrap();
    let d = it.next().unwrap().parse::<usize>().unwrap();
    println!("{}", shoes(a, b, c, d));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(shoes(1, 7, 3, 3), 1);
    }
    #[test]
    fn ex_2() {
        assert_eq!(shoes(7, 7, 7, 7), 3);
    }
}

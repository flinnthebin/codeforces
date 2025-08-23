use std::cmp::max;
use std::io::{self, Read};

fn sum(a: usize, b: usize, c: usize) -> &'static str {
    let s = a + b + c;
    let m = max(a, max(b, c));
    if m == s - m { "YES" } else { "NO" }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        let c = it.next().unwrap().parse::<usize>().unwrap();
        println!("{}", sum(a, b, c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(sum(1, 4, 3), "YES")
    }
    #[test]
    fn ex_2() {
        assert_eq!(sum(2, 5, 8), "NO")
    }
    #[test]
    fn ex_3() {
        assert_eq!(sum(9, 11, 20), "YES")
    }
    #[test]
    fn ex_4() {
        assert_eq!(sum(0, 0, 0), "YES")
    }
    #[test]
    fn ex_5() {
        assert_eq!(sum(20, 20, 20), "NO")
    }
    #[test]
    fn ex_6() {
        assert_eq!(sum(4, 12, 3), "NO")
    }
    #[test]
    fn ex_7() {
        assert_eq!(sum(15, 7, 8), "YES")
    }
}

use std::io::{self, Read};

struct Scanner<'a> {
    it: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            it: s.split_ascii_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse().ok().unwrap()
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

fn maple(a: i64, b: i64) -> i64 {
    if a == b {
        0
    } else {
        let x = gcd(a, b);
        if x == a || x == b { 1 } else { 2 }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: i64 = sc.next();
    for _ in 0..t {
        let a: i64 = sc.next();
        let b: i64 = sc.next();
        println!("{}", maple(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(maple(1, 2), 1);
    }
    #[test]
    fn ex_2() {
        assert_eq!(maple(10, 3), 2);
    }
    #[test]
    fn ex_3() {
        assert_eq!(maple(1000, 1000), 0);
    }
}

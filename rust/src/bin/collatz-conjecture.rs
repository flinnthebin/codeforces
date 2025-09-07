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

fn collatz(k: usize, x: usize) -> usize {
    x << k
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: usize = sc.next();
    for _ in 0..t {
        let k: usize = sc.next();
        let x: usize = sc.next();
        println!("{}", collatz(k, x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(collatz(1, 4), 1);
    }
    #[test]
    fn ex_2() {
        assert_eq!(collatz(1, 5), 10);
    }
    #[test]
    fn ex_3() {
        assert_eq!(collatz(5, 4), 21);
    }
}

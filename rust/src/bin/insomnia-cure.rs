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

fn damaged_dragons(k: u32, l: u32, m: u32, n: u32, d: u32) -> usize {
    let dmg = [k, l, m, n];
    (1..=d).filter(|&x| dmg.iter().any(|&p| x % p == 0)).count()
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let k: u32 = sc.next();
    let l: u32 = sc.next();
    let m: u32 = sc.next();
    let n: u32 = sc.next();
    let d: u32 = sc.next();
    println!("{}", damaged_dragons(k, l, m, n, d));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(damaged_dragons(1, 2, 3, 4, 12), 12)
    }
    #[test]
    fn ex_2() {
        assert_eq!(damaged_dragons(2, 3, 4, 5, 24), 17)
    }
}

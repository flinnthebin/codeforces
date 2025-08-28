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

fn valid(x: usize, y: usize) -> bool {
    let (first, second) = if x > y { (x, y) } else { (y, x) };
    first <= 2 * second + 2
}

fn dream(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if a > c || b > d {
        return "NO";
    }
    let halftime = valid(a, b);
    let fulltime = valid(c - a, d - b);
    if halftime && fulltime { "YES" } else { "NO" }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: usize = sc.next();
    for _ in 0..n {
        let a: usize = sc.next();
        let b: usize = sc.next();
        let c: usize = sc.next();
        let d: usize = sc.next();
        println!("{}", dream(a, b, c, d));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(dream(1, 4, 1, 4), "YES")
    }
    #[test]
    fn ex_2() {
        assert_eq!(dream(4, 1, 4, 1), "YES")
    }
    #[test]
    fn ex_3() {
        assert_eq!(dream(1, 4, 2, 5), "YES")
    }
    #[test]
    fn ex_4() {
        assert_eq!(dream(0, 100, 0, 100), "NO")
    }
    #[test]
    fn ex_5() {
        assert_eq!(dream(1, 4, 2, 9), "NO")
    }
    #[test]
    fn ex_6() {
        assert_eq!(dream(3, 1, 13, 5), "YES")
    }
    #[test]
    fn ex_7() {
        assert_eq!(dream(8, 11, 17, 36), "NO")
    }
    #[test]
    fn ex_8() {
        assert_eq!(dream(19, 41, 30, 50), "NO")
    }
    #[test]
    fn ex_9() {
        assert_eq!(dream(20, 38, 30, 60), "YES")
    }
    #[test]
    fn ex_10() {
        assert_eq!(dream(0, 0, 0, 0), "YES")
    }
    #[test]
    fn ex_11() {
        assert_eq!(dream(100, 100, 100, 100), "YES")
    }
}

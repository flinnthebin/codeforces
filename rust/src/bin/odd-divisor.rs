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

fn odd(mut x: u64) -> &'static str {
    if x & 1 == 1 {
        "YES"
    } else {
        while x > 2 {
            x /= 2;
            if x & 1 == 1 {
                return "YES";
            }
        }
        "NO"
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: u64 = sc.next();
    for _ in 0..n {
        let x: u64 = sc.next();
        println!("{}", odd(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(odd(2), "NO");
    }
    #[test]
    fn ex_2() {
        assert_eq!(odd(3), "YES");
    }
    #[test]
    fn ex_3() {
        assert_eq!(odd(4), "NO");
    }
    #[test]
    fn ex_4() {
        assert_eq!(odd(5), "YES");
    }
    #[test]
    fn ex_5() {
        assert_eq!(odd(998244353), "YES");
    }
    #[test]
    fn ex_6() {
        assert_eq!(odd(1099511627776), "NO");
    }
}

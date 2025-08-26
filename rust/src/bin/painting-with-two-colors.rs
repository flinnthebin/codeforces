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

fn symmetrical(n: u64, a: u64, b: u64) -> &'static str {
    if (n - b) & 1 == 1 {
        "NO"
    } else if a <= b {
        "YES"
    } else if (n - a) & 1 == 0 {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: u64 = sc.next();
    for _ in 0..n {
        let n: u64 = sc.next();
        let a: u64 = sc.next();
        let b: u64 = sc.next();
        println!("{}", symmetrical(n, a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(symmetrical(5, 3, 1), "YES");
    }
    #[test]
    fn ex_2() {
        assert_eq!(symmetrical(4, 1, 2), "YES");
    }
    #[test]
    fn ex_3() {
        assert_eq!(symmetrical(7, 7, 4), "NO");
    }
    #[test]
    fn ex_4() {
        assert_eq!(symmetrical(8, 3, 7), "NO");
    }
    #[test]
    fn ex_5() {
        assert_eq!(symmetrical(1, 1, 1), "YES");
    }
    #[test]
    fn ex_6() {
        assert_eq!(symmetrical(1000000000, 1000000000, 1000000000), "YES");
    }
    #[test]
    fn ex_7() {
        assert_eq!(symmetrical(3, 2, 1), "NO");
    }
}

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

macro_rules! yesno {
    ($cond:expr) => {
        if $cond { "YES" } else { "NO" }
    };
}

fn yes(s: &str) -> &'static str {
    let s = s.to_lowercase();
    yesno!(s == "yes")
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: usize = sc.next();
    for _ in 0..t {
        let x: String = sc.next();
        println!("{}", yes(&x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(yes("YES"), "YES");
    }
    #[test]
    fn ex_2() {
        assert_eq!(yes("yES"), "YES");
    }
    #[test]
    fn ex_3() {
        assert_eq!(yes("yes"), "YES");
    }
    #[test]
    fn ex_4() {
        assert_eq!(yes("Yes"), "YES");
    }
    #[test]
    fn ex_5() {
        assert_eq!(yes("YeS"), "YES");
    }
    #[test]
    fn ex_6() {
        assert_eq!(yes("Noo"), "NO");
    }
    #[test]
    fn ex_7() {
        assert_eq!(yes("orZ"), "NO");
    }
    #[test]
    fn ex_8() {
        assert_eq!(yes("yEz"), "NO");
    }
    #[test]
    fn ex_9() {
        assert_eq!(yes("Yas"), "NO");
    }
    #[test]
    fn ex_10() {
        assert_eq!(yes("XES"), "NO");
    }
}

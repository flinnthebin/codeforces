use std::cmp::min;
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

fn toasts(sc: &mut Scanner) -> usize {
    let n: usize = sc.next(); // friends
    let k: usize = sc.next(); // bottles
    let l: usize = sc.next(); // millilitres
    let c: usize = sc.next(); // limes
    let d: usize = sc.next(); // slices of lime
    let p: usize = sc.next(); // grams of salt
    let nl: usize = sc.next(); // required ml of drink
    let np: usize = sc.next(); // required g of salt

    let drinks = (k * l) / nl;
    let limes = c * d;
    let salt = p / np;

    min(drinks, min(limes, salt)) / n
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    println!("{}", toasts(&mut sc));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        let data = "3 4 5 10 8 100 3 1".to_string();
        let mut sc = Scanner::new(&data);
        assert_eq!(toasts(&mut sc), 2);
    }
    #[test]
    fn ex_2() {
        let data = "5 100 10 1 19 90 4 3".to_string();
        let mut sc = Scanner::new(&data);
        assert_eq!(toasts(&mut sc), 3);
    }
    #[test]
    fn ex_3() {
        let data = "10 1000 1000 25 23 1 50 1".to_string();
        let mut sc = Scanner::new(&data);
        assert_eq!(toasts(&mut sc), 0);
    }
}

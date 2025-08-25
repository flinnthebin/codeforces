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

fn taxi(n: usize, sc: &mut Scanner) -> usize {
    let (mut one, mut two, mut three, mut four) = (0usize, 0usize, 0usize, 0usize);
    for _ in 0..n {
        match sc.next::<usize>() {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            4 => four += 1,
            _ => unreachable!(),
        }
    }

    let mut taxis = four;
    taxis += three;
    if one >= three {
        one -= three;
    } else {
        one = 0;
    }

    taxis += two / 2;
    two %= 2;

    if two == 1 {
        taxis += 1;
        if one >= 2 {
            one -= 2;
        } else {
            one = 0;
        }
    }

    taxis += one.div_ceil(4);

    taxis
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: usize = sc.next();
    println!("{}", taxi(n, &mut sc));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        let data = "5 1 2 4 3 3".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(taxi(n, &mut sc), 4);
    }
    #[test]
    fn ex_2() {
        let data = "8 2 3 4 4 2 1 3 1".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(taxi(n, &mut sc), 5);
    }
    #[test]
    fn edge() {
        let data = "3 3 3 2".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(taxi(n, &mut sc), 3);
    }
    #[test]
    fn group() {
        let data = "4 1 1 1 1".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(taxi(n, &mut sc), 1);
    }
}

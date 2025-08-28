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

fn neat(n: usize, v: &[usize]) -> usize {
    let mut counter: usize = 0;
    let mut cur: usize = 0;
    let mut prev: usize = v[0];
    let mut res: usize = 0;
    for x in v {
        if *x == cur {
            counter += 1
        }
        if counter == cur {
            res += counter;
        }
    }
    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut v = Vec::new();
        for _ in 0..n {
            let x: usize = sc.next();
            v.push(x);
        }
        println!("{}", neat(n, &v));
    }
}

use std::io::{self, Read};

fn marathon(a: usize, b: usize, c: usize, d: usize) -> usize {
    let mut count = 0;
    for e in [b, c, d] {
        if e > a {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        let c = it.next().unwrap().parse::<usize>().unwrap();
        let d = it.next().unwrap().parse::<usize>().unwrap();
        println!("{}", marathon(a, b, c, d));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(marathon(2, 3, 4, 1), 2)
    }
    #[test]
    fn ex_2() {
        assert_eq!(marathon(10000, 0, 1, 2), 0)
    }
    #[test]
    fn ex_3() {
        assert_eq!(marathon(500, 600, 400, 300), 1)
    }
    #[test]
    fn ex_4() {
        assert_eq!(marathon(0, 9999, 10000, 9998), 3)
    }
}

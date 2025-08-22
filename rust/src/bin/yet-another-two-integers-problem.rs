use std::io::{self, Read};

fn two_ints(a: usize, b: usize) -> usize {
    let d = a.abs_diff(b);
    d.div_ceil(10)
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();

    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        println!("{}", two_ints(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(two_ints(5, 5), 0);
    }
    #[test]
    fn ex_2() {
        assert_eq!(two_ints(13, 42), 3);
    }
    #[test]
    fn ex_3() {
        assert_eq!(two_ints(18, 4), 2);
    }
    #[test]
    fn ex_4() {
        assert_eq!(two_ints(1337, 420), 92);
    }
    #[test]
    fn ex_5() {
        assert_eq!(two_ints(123456789, 1000000000), 87654322);
    }
    #[test]
    fn ex_6() {
        assert_eq!(two_ints(100500, 9000), 9150);
    }
}

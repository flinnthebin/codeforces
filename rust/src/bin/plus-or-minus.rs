use std::io::{self, Read};

fn plus_or_minus(a: isize, b: isize, c: isize) -> String {
    if a + b == c {
        "+".to_string()
    } else {
        "-".to_string()
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();

    let t = it.next().unwrap().parse::<isize>().unwrap();

    for _ in 0..t {
        let a = it.next().unwrap().parse::<isize>().unwrap();
        let b = it.next().unwrap().parse::<isize>().unwrap();
        let c = it.next().unwrap().parse::<isize>().unwrap();
        println!("{}", plus_or_minus(a, b, c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(plus_or_minus(1, 2, 3), "+");
    }
    #[test]
    fn ex_2() {
        assert_eq!(plus_or_minus(3, 2, 1), "-");
    }
    #[test]
    fn ex_3() {
        assert_eq!(plus_or_minus(2, 9, -7), "-");
    }
    #[test]
    fn ex_4() {
        assert_eq!(plus_or_minus(3, 4, 7), "+");
    }
    #[test]
    fn ex_5() {
        assert_eq!(plus_or_minus(1, 1, 2), "+");
    }
    #[test]
    fn ex_6() {
        assert_eq!(plus_or_minus(1, 1, 0), "-");
    }
    #[test]
    fn ex_7() {
        assert_eq!(plus_or_minus(3, 3, 6), "+");
    }
    #[test]
    fn ex_8() {
        assert_eq!(plus_or_minus(9, 9, 18), "+");
    }
    #[test]
    fn ex_9() {
        assert_eq!(plus_or_minus(9, 9, 0), "-");
    }
    #[test]
    fn ex_10() {
        assert_eq!(plus_or_minus(1, 9, -8), "-");
    }
    #[test]
    fn ex_11() {
        assert_eq!(plus_or_minus(1, 9, 10), "+");
    }
}

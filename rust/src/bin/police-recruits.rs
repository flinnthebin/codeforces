use std::io::{self, Read};

fn chrono(s: &str) -> usize {
    let mut it = s.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut count = 0;
    let mut officers = 0;
    for _ in 0..n {
        let event = it.next().unwrap().parse::<isize>().unwrap();
        if event == -1 {
            if officers > 0 {
                officers -= 1;
            } else {
                count += 1;
            }
        } else {
            officers += event;
        }
    }
    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", chrono(&buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(chrono("3 -1 -1 1"), 2);
    }
    #[test]
    fn ex_2() {
        assert_eq!(chrono("8 1 -1 1 -1 -1 1 1 1"), 1);
    }
    #[test]
    fn ex_3() {
        assert_eq!(chrono("11 -1 -1 2 -1 -1 -1 -1 -1 -1 -1 -1"), 8);
    }
}

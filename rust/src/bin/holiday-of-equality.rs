use std::io::{self, Read};

fn min_burles(s: String) -> usize {
    let mut it = s.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut v = Vec::<usize>::new();

    for _ in 0..n {
        let x = it.next().unwrap().parse::<usize>().unwrap();
        v.push(x);
    }

    let mut count = 0;
    let max = *v.iter().max().unwrap();

    for val in v {
        count += max - val
    }

    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let res = min_burles(buf);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(min_burles("5 0 1 2 3 4".to_string()), 10);
    }
    #[test]
    fn example_2() {
        assert_eq!(min_burles("5 1 1 0 1 1".to_string()), 1);
    }
    #[test]
    fn example_3() {
        assert_eq!(min_burles("3 1 3 1".to_string()), 4);
    }
    #[test]
    fn example_4() {
        assert_eq!(min_burles("1 12".to_string()), 0);
    }
}

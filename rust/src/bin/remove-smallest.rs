use std::io::{self, Read};

fn remove(v: &[usize]) -> &'static str {
    for w in v.windows(2) {
        if w[1].abs_diff(w[0]) > 1 {
            return "NO";
        }
    }
    "YES"
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = it.next().unwrap().parse::<usize>().unwrap();
        let mut v = Vec::<usize>::new();
        for _ in 0..n {
            v.push(it.next().unwrap().parse::<usize>().unwrap());
            v.sort();
        }
        println!("{}", remove(&v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(remove(&[1, 2, 2]), "YES");
    }
    #[test]
    fn ex_2() {
        assert_eq!(remove(&[5, 5, 5, 5]), "YES");
    }
    #[test]
    fn ex_3() {
        assert_eq!(remove(&[1, 2, 4]), "NO");
    }
    #[test]
    fn ex_4() {
        assert_eq!(remove(&[1, 3, 4, 4]), "NO");
    }
    #[test]
    fn ex_5() {
        assert_eq!(remove(&[100]), "YES");
    }
}

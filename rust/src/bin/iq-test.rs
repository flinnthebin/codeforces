use std::io::{self, Read};

fn differs(v: &[usize]) -> usize {
    let even = v.iter().filter(|&&x| x % 2 == 0).count();
    let odd = v.len() - even;

    if even > odd {
        v.iter().position(|&x| x % 2 != 0).unwrap() + 1
    } else {
        v.iter().position(|&x| x % 2 == 0).unwrap() + 1
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    _ = it.next().unwrap().parse::<usize>().unwrap();
    let v: Vec<usize> = it.map(|x| x.parse::<usize>().unwrap()).collect();
    println!("{}", differs(&v));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(differs(&[2, 4, 7, 8, 10]), 3);
    }
    #[test]
    fn ex_2() {
        assert_eq!(differs(&[1, 2, 1, 1]), 2);
    }
}

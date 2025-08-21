use std::io::{self, Read};

fn str_repeat(s: &str, m: usize) -> String {
    s.repeat(m)
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();

    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();

    for x in 1..=n {
        if x % 2 != 0 {
            println!("{}", str_repeat("#", m));
        } else if (x / 2) % 2 == 0 {
            println!("#{}", str_repeat(".", m - 1));
        } else {
            println!("{}#", str_repeat(".", m - 1));
        }
    }
}

use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a: Vec<i64> = (0..n)
            .map(|_| it.next().unwrap().parse().unwrap())
            .collect();
        let b: Vec<i64> = (0..n)
            .map(|_| it.next().unwrap().parse().unwrap())
            .collect();

        let mut count = 1;
        //#[allow(clippy::while_let_loop)]
        loop {
            if let Some(i) = (0..n).find(|&i| a[i] > b[i]) {
                a[i] -= 1;
            } else {
                break;
            }
            if let Some(j) = (0..n).find(|&j| a[j] < b[j]) {
                a[j] += 1;
            }
            count += 1;
        }
        println!("{}", count);
    }
}

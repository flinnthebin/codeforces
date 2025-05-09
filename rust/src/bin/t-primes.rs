macro_rules! read_stdin {
    ($t:ty) => {{
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

fn t_prime(x: u64, isprime: &[bool]) -> bool {
    if x < 4 {
        return false;
    }
    let sqrt_x = (x as f64).sqrt() as u64;
    sqrt_x * sqrt_x == x && isprime[sqrt_x as usize]
}

fn main() {
    let mut v: Vec<u64> = read_stdin!(u64);
    let mut it = v.drain(..);

    let limit: usize = 1000001;
    let mut isprime = vec![true; limit];
    isprime[0] = false;
    isprime[1] = false;
    for i in 2..=((limit as f64).sqrt() as usize) {
        if isprime[i] {
            let mut j = i * i;
            while j < limit {
                isprime[j] = false;
                j += i;
            }
        }
    }

    while let Some(n) = it.next() {
        let xs: Vec<u64> = it.by_ref().take(n as usize).collect();
        for x in xs {
            if t_prime(x, &isprime) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

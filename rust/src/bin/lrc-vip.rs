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

fn binary_gcd(mut a: usize, mut b: usize) -> usize {
    let mut d = 0;
    while a % 2 == 0 && b % 2 == 0 {
        a /= 2;
        b /= 2;
        d += 1;
    }
    while a % 2 == 0 {
        a /= 2;
    }
    while b % 2 == 0 {
        b /= 2;
    }
    if a > b {
        a -= b;
        while a % 2 == 0 {
            a /= 2;
        }
    }
    if a > b {
        b -= a;
        while b % 2 == 0 {
            b /= 2;
        }
    }
    a * (1 << d)
}

fn solve(xs: Vec<usize>) {
    let n = xs.len();
    let mut arr = xs.clone();
    arr.sort();

    let smallest = arr[0];

    let mut group = vec![2; n];
    let mut used_smallest = false;

    for (i, &val) in xs.iter().enumerate() {
        if val == smallest && !used_smallest {
            group[i] = 1;
            used_smallest = true;
        }
    }

    let mut gcd_b = 0;
    let mut gcd_c = 0;
    for (i, &val) in xs.iter().enumerate() {
        if group[i] == 1 {
            gcd_b = if gcd_b == 0 {
                val
            } else {
                binary_gcd(gcd_b, val)
            };
        } else {
            gcd_c = if gcd_c == 0 {
                val
            } else {
                binary_gcd(gcd_c, val)
            };
        }
    }

    if gcd_b != gcd_c {
        println!("YES");
        for &g in &group {
            print!("{} ", g);
        }
        println!();
    } else {
        println!("NO");
    }
}

fn main() {
    let mut v = read_stdin!(usize);
    let mut it = v.drain(..);

    let _t = it.next().unwrap();
    while let Some(n) = it.next() {
        let xs: Vec<usize> = it.by_ref().take(n).collect();
        solve(xs);
    }
}

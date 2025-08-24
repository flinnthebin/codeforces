#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{self, Read};

// ============================ SCANNER ===========================
#[allow(dead_code)]
struct Scanner<'a> {
    it: std::str::SplitAsciiWhitespace<'a>,
}

#[allow(dead_code)]
impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            it: s.split_ascii_whitespace(),
        }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse().ok().unwrap()
    }
}

// ============================ MACROS ============================
#[allow(unused_macros)]
macro_rules! dbg {
    ($($x:expr), * $(,)?) => { eprintln!($(concat!(stringify!($x), "={:?} "), $x),*); };
}

#[allow(unused_macros)]
macro_rules! chmin {
    ($a:expr, $b:expr) => {{
        let b = $b;
        if b < $a {
            $a = b;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($a:expr, $b: expr) => {{
        let b = $b;
        if b > $a {
            $a = b;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! yesno {
    ($cond:expr) => {
        if $cond { "YES" } else { "NO" }
    };
}

// ============================ MATH ==============================
#[allow(dead_code)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

#[allow(dead_code)]
fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (1, 0, a)
    } else {
        let (x, y, g) = ext_gcd(b, a % b);
        (y, x - (a / b) * y, g)
    }
}

#[allow(dead_code)]
fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    a = ((a % m) + m) % m;
    let mut r = 1i64 % m;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % m;
        }
        a = (a * a) % m;
        e >>= 1;
    }
    r
}

#[allow(dead_code)]
fn mod_inv(a: i64, m: i64) -> i64 {
    let (x, _, g) = ext_gcd(a, m);
    assert!(g == 1 || g == -1, "mod inv requires gcd(a, m) = 1");
    ((x % m) + m) % m
}

// ======================== PREFIX SUMS ===========================
#[allow(dead_code)]
fn prefix_sums(a: &[i64]) -> Vec<i64> {
    let mut ps = Vec::with_capacity(a.len() + 1);
    ps.push(0);
    for &x in a {
        ps.push(ps.last().unwrap() + x);
    }
    ps
}

// ======================= BINARY SEARCH ==========================
// Vanilla binary search
#[allow(dead_code)]
fn binary_search<T: Ord>(slice: &[T], key: &T) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if key < &slice[mid] {
            hi = mid;
        } else if &slice[mid] < key {
            lo = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

// ================== LOWER/UPPER BOUND ===========================
// Monotonic predicate binary search, on sorted arrays
// // - lower_bound:  first index i such that slice[i] >= key
#[allow(dead_code)]
fn lower_bound<T: Ord>(slice: &[T], key: &T) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if &slice[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo < slice.len() { Some(lo) } else { None }
}

// - upper_bound:  first index i such that slice[i] >  key
#[allow(dead_code)]
fn upper_bound<T: Ord>(slice: &[T], key: &T) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if &slice[mid] <= key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo < slice.len() { Some(lo) } else { None }
}

// ============================ BFS ===============================

#[allow(dead_code)]
fn bfs(start: usize, graph: &[Vec<usize>]) -> Vec<i32> {
    let n = graph.len();
    let mut visited = vec![-1; n];
    let mut q = VecDeque::new();
    visited[start] = 0;
    q.push_back(start);
    while let Some(v) = q.pop_front() {
        let d = visited[v];
        for &x in &graph[v] {
            if visited[x] == -1 {
                visited[x] = d + 1;
                q.push_back(x);
            }
        }
    }
    visited
}

// ============================ DFS ===============================

#[allow(dead_code)]
fn dfs(start: usize, graph: &[Vec<usize>]) -> Vec<bool> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut stack = vec![start];
    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        for &x in &graph[v] {
            if !visited[x] {
                stack.push(x);
            }
        }
    }
    visited
}

// =========================== SIEVE ==============================
#[allow(dead_code)]
fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is = vec![true; n + 1];
    is[0] = false;
    is[1] = false;
    let mut p = 2usize;
    while p * p <= n {
        if is[p] {
            let mut j = p * p;
            while j <= n {
                is[j] = false;
                j += p;
            }
        }
        p += 1;
    }
    let mut ps = Vec::new();
    for i in 2..=n {
        if is[i] {
            ps.push(i);
        }
    }
    ps
}

// ============================ DSU ===============================

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
struct DSU {
    parent: Vec<i32>,
    size: Vec<i32>,
}

#[allow(dead_code)]
impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n as i32).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] as usize != x {
            let r = self.find(self.parent[x] as usize);
            self.parent[x] = r as i32;
        }
        self.parent[x] as usize
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut x = self.find(a);
        let mut y = self.find(b);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x as i32;
        self.size[x] += self.size[y];
        true
    }
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    fn size(&mut self, a: usize) -> i32 {
        let r = self.find(a);
        self.size[r]
    }
}

// ========================== FENWICK =============================

#[allow(dead_code)]
struct Fenwick {
    n: usize,
    bit: Vec<i64>, // 1-based
}

#[allow(dead_code)]
impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }
    fn add(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & (!idx + 1); // idx += & -idx
        }
    }
    fn sum(&self, mut idx: usize) -> i64 {
        let mut s = 0i64;
        while idx > 0 {
            s += self.bit[idx];
            idx -= idx & (!idx + 1); // idx -= & -idx
        }
        s
    }
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        // inclusive l..=r, with 1-based indices
        self.sum(r) - self.sum(l - 1)
    }
}

// ======================== MODINT ================================
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct ModInt<const M: i64> {
    v: i64,
}

#[allow(dead_code)]
impl<const M: i64> ModInt<M> {
    fn new(x: i64) -> Self {
        let mut x = x % M;
        if x < 0 {
            x += M;
        }
        Self { v: x }
    }
    fn pow(self, mut e: i64) -> Self {
        let mut base = self;
        let mut res = Self::new(1);
        while e > 0 {
            if e & 1 == 1 {
                res *= base;
            }
            base *= base;
            e >>= 1;
        }
        res
    }
    fn inv(self) -> Self {
        // M must be prime for Fermat’s little theorem
        self.pow(M - 2)
    }
    fn val(self) -> i64 {
        self.v
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
impl<const M: i64> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut x = self.v + rhs.v;
        if x >= M {
            x -= M;
        }
        Self { v: x }
    }
}
impl<const M: i64> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const M: i64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut x = self.v - rhs.v;
        if x < 0 {
            x += M;
        }
        Self { v: x }
    }
}
impl<const M: i64> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<const M: i64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new((self.v * rhs.v) % M)
    }
}
impl<const M: i64> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<const M: i64> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl<const M: i64> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// ===================== COMBINATORICS ===========================
#[allow(dead_code)]
struct Comb<const M: i64> {
    fact: Vec<ModInt<M>>,
    ifact: Vec<ModInt<M>>,
}

#[allow(dead_code)]
impl<const M: i64> Comb<M> {
    fn new(n: usize) -> Self {
        let mut fact = vec![ModInt::<M>::new(1); n + 1];
        let mut ifact = vec![ModInt::<M>::new(1); n + 1];

        for i in 1..=n {
            fact[i] = fact[i - 1] * ModInt::<M>::new(i as i64);
        }
        ifact[n] = fact[n].inv();
        for i in (1..=n).rev() {
            ifact[i - 1] = ifact[i] * ModInt::<M>::new(i as i64);
        }
        Self { fact, ifact }
    }

    fn ncr(&self, n: usize, r: usize) -> ModInt<M> {
        if r > n {
            return ModInt::<M>::new(0);
        }
        self.fact[n] * self.ifact[r] * self.ifact[n - r]
    }

    fn npr(&self, n: usize, r: usize) -> ModInt<M> {
        if r > n {
            return ModInt::<M>::new(0);
        }
        self.fact[n] * self.ifact[n - r]
    }
}

// ============================ MAIN ==============================

fn main() {}

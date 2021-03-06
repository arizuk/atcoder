#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;

type Matrix = Vec<Vec<u64>>;

// Matrix Exponentiation
fn mat_mul(a: &Matrix, b: &Matrix, m: u64) -> Matrix {
    assert!(a[0].len() == b.len());

    let h = a.len();
    let w = b[0].len();

    let mut ret = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            for k in 0..b.len() {
                ret[i][j] += (a[i][k] * b[k][j]) % m;
                ret[i][j] %= m;
            }
        }
    }
    ret
}

fn mat_pow(a: &Matrix, mut n: u64, m: u64) -> Matrix {
    let mut a = a.clone();
    let mut b = vec![vec![0; a.len()]; a.len()];
    for i in 0..a.len() {
        b[i][i] = 1;
    }
    while n>0 {
        if n&1 > 0 {
            b = mat_mul(&b, &a, m);
        }
        a = mat_mul(&a, &a, m);
        n /= 2;
    }
    b
}

pub fn mod_pow(b: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        return 1;
    }
    let mut ret = mod_pow(b * b % m, p / 2, m) % m;
    if p % 2 == 1 {
        ret = ret * b % m;
    }
    ret
}

fn main() {
    input!{
      l: u64,
      a: u64,
      b: u64,
      m: u64,
    }
    const D: usize = 18;

    let mut acm = vec![l; D+1];
    acm[0] = 0;
    for d in 1..D+1 {
        let ten = 10u64.pow(d as u32);
        if a > ten {
            acm[d] = 0;
            continue;
        }
        let idx = (ten - a + b - 1) / b;
        acm[d] = min(idx, l);
    }

    let mut v = vec![vec![0, a, 1]];
    for d in 1..D+1 {
        let td = 10u64.pow(d as u32) % m;
        let r = vec![
            vec![td,0,0],
            vec![1,1,0],
            vec![0,b,1],
        ];
        let cd = acm[d] - acm[d-1];
        let power = mat_pow(&r, cd, m);
        v = mat_mul(&v, &power, m);
        // debug!(v);
    }
    println!("{}", v[0][0]);
}

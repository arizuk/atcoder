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

pub fn gen_prime_table(n: u64) -> Vec<bool> {
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n as usize {
        if is_prime[i] {
            let mut j = 2;
            while i * j <= n as usize {
                is_prime[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    is_prime
}

fn main() {
    input!{
      q: usize,
      lrs: [(usize, usize); q],
    }
    let m = 1e5 as usize + 1;
    let primes = gen_prime_table(m as _);
    let mut primes2 = vec![false; m];
    for i in 0..primes.len() {
        if i%2 == 0 || !primes[i] {
            continue;
        }
        if primes[ (i+1)/2 ] {
            primes2[i] = true;
        }
    }

    let mut s = vec![0; m+1];
    for i in 0..m {
        s[i+1] = s[i] + if primes2[i] { 1 } else { 0 };
    }

    for i in 0..q {
        let (l, r) = lrs[i];
        println!("{}", s[r+1] - s[l]);
    }
}

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



fn main() {
    input!{
      n: usize,
      x: usize,
      mut blus: [(i64, i64, i64); n]
    }
    let d: i64 = blus.iter().map(|v| -v.0 * v.1).sum();
    let mut sums: Vec<(usize, i64)> = blus.iter().map(|v| v.2 * (x as i64 - v.0) + v.1 * v.0).enumerate().collect();
    sums.sort_by_key(|v| v.1);
    sums.reverse();

    let mut acm = vec![0; n];
    let mut sum_index_of = vec![0; n];
    for (j, &(i, sum)) in sums.iter().enumerate() {
        sum_index_of[i] = j;
        if j > 0 {
            acm[j] = acm[j-1] + sum;
        } else {
            acm[j] = sum;
        }
    }
    let ok = |k: usize| {
        let q: i64 = (k/x) as i64;
        let r: i64 = k as i64 - (q*x as i64);
        for i in 0..n {
            let mut d = d.clone();
            let &(b, l, u) = &blus[i];
            d += l as i64 * min(r, b);
            d += u as i64 * max(r-b, 0);

            let tmp = l as i64 * min(r, b) +u as i64 * max(r-b, 0);

            // q個足す
            if q > 0 {
                d += acm[(q-1) as usize];
                if r > 0 {
                    let j = sum_index_of[i];
                    if j <= (q-1) as usize {
                        // debug!(i, j, k, q, r, sums.len());
                        d += sums[q as usize].1;
                        d -= sums[j].1;
                    }
                }
            }

            if d >= 0 {
                return true;
            }
        }
        return false
    };

    let mut l = 0;
    let mut r = n*x+1;
    while l != r {
        let k = (l+r)/2;
        if ok(k) {
            // debug!(l, r, k, "Ok");
            r = k;
        } else {
            // debug!(l, r, k, "Ng");
            l = k + 1;
        }
    }
    println!("{}", r);
}

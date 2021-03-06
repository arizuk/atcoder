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
use std::io::{stdout, stdin, BufWriter, Write};


fn check_needed(aa: &Vec<usize>, k: usize, i: usize) -> bool {
    let n = aa.len();
    let mut dp = vec![vec![false; k+1]; n+1];
    dp[0][0] = true;
    for j in 0..n {
        let a = aa[j];
        if i == j {
            for cur_k in 0..k {
                dp[j+1][cur_k] |= dp[j][cur_k];
            }
        } else {
            for cur_k in 0..k {
                let idx = min(k, cur_k+a);
                dp[j+1][cur_k] |= dp[j][cur_k];
                dp[j+1][idx] |= dp[j][cur_k];
            }
        }
    }

    let lower = if k>aa[i] { k-aa[i] } else { 0 };
    // for i in 0..n {
    //     debug!(i, dp[i]);
    // }

    // debug!(i, lower, aa[i], k);
    // debug!(&dp[n][lower..k]);
    for j in lower..k {
        // iを入れるとKを超える集合が存在する=必要なカード
        if dp[n][j] {
            return true
        }
    }
    false
}

#[doc = " [l, r)"]
pub fn binary_search_by<F>(mut l: usize, mut r: usize, f: &F) -> usize
where
    F: Fn(usize) -> bool,
{
    assert!(l <= r);
    while r != l {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      k: usize,
      mut aa: [usize; n],
    }
    aa.sort();

    let mut ok = n;
    let mut ng = 0;
    while ok != ng {
        let mid = (ok + ng) / 2;
        if check_needed(&aa, k, mid) {
            ok = mid;
        } else {
            ng = mid + 1;
        }
    }

    // debug!(ans);
    puts!("{}\n", ok);
}

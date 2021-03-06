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

const INF: i64 = -1 * (1 << 60);

fn dp(vs: &Vec<i64>, memo: &mut Vec<Vec<Vec<Option<i64>>>>, l: usize, r: usize, k: i64) -> i64 {
    if k < 0 || l > r {
        return INF
    }
    if k == 0 {
        return 0;
    }
    if l == r {
        return max(0, vs[l])
    }

    if let Some(v) = memo[l][r][k as usize] {
        return v
    }
    let l1 = dp(vs, memo, l+1, r, k-1) + vs[l];
    let l2 = dp(vs, memo, l+1, r, k-2);
    let r1 = dp(vs, memo, l, r-1, k-1) + vs[r];
    let r2 = dp(vs, memo, l, r-1, k-2);

    let k = k as usize;
    memo[l][r][k] = Some(max(max(l1, max(max(l2, r1), r2)), 0));
    memo[l][r][k].unwrap()
}

fn main() {
    input!{
      n: usize,
      k: usize,
      vs: [i64; n],
    }
    let mut memo = vec![vec![vec![None; k+1]; n]; n];
    let ans = dp(&vs, &mut memo, 0, n-1, k as _);
    println!("{}", ans);
}

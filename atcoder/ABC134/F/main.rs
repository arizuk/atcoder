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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      k: usize,
    }

    const MOD: usize = 1e9 as usize + 7;

    // dp[i][j][k]
    let mut dp = vec![vec![vec![0; k+1]; n+1]; n+1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..n+1 {
            for l in 0..k+1 {
                if l+2*j > k {
                    continue;
                }

                // 保留
                if j+1 <= n {
                    dp[i+1][j+1][l + 2*j] += dp[i][j][l];
                    dp[i+1][j+1][l + 2*j] %= MOD;
                }

                // iを埋める
                dp[i+1][j][l + 2*j] += dp[i][j][l];
                dp[i+1][j][l + 2*j] %= MOD;

                // 保留で箱iを埋める / i は保留
                dp[i+1][j][l + 2*j] += dp[i][j][l] * j % MOD;
                dp[i+1][j][l + 2*j] %= MOD;

                if j > 0 {
                    // 保留で箱iを埋める / i を別の箱にいれる
                    dp[i+1][j-1][l + 2*j] += dp[i][j][l] * j % MOD * j % MOD;
                    dp[i+1][j-1][l + 2*j] %= MOD;
                }

                // i を別の箱にいれる
                dp[i+1][j][l + 2*j] += dp[i][j][l] * j % MOD;
                dp[i+1][j][l + 2*j] %= MOD;
            }
        }
    }
    puts!("{}\n", dp[n][0][k]);
}

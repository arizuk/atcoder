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
type I = usize;

fn main() {
    input!{
      n: usize,
      aa: [usize; n]
    }

    let mut left_len = vec![0; n];
    let mut right_len = vec![0; n];

    let mut len = 0;
    let mut start = 0;
    for i in 1..n {
        if aa[i] > aa[i-1] {
            len += 1;
        } else {
            for j in start..i {
                left_len[j] = len;
                if len == 0 {
                    break;
                }
                len -= 1;
            }
            start = i;
        }
    }

    let mut len = 0;
    let mut start = n-1;
    for i in (0..n-1).rev() {
        if aa[i] > aa[i+1] {
            len += 1;
        } else {
            for j in (i+1..start+1).rev() {
                right_len[j] = len;
                if len == 0 {
                    break;
                }
                len -= 1;
            }
            start = i;
        }
    }

    let mut l = 0;
    let mut r = n-1;
    let mut last = 0;
    let mut ans = vec![];
    while r >= l {
        let lok = aa[l] > last;
        let rok = aa[r] > last;

        if !(lok || rok) {
            break;
        }

        if lok && rok {
            if l == r {
                last = aa[l];
                l += 1;
                ans.push("L");
                break;
            }

            if aa[l] < aa[r] {
                last = aa[l];
                l += 1;
                ans.push("L");
            } else if aa[l] == aa[r] {
                if left_len[l] > right_len[r] {
                    last = aa[l];
                    l += 1;
                    ans.push("L");
                } else {
                    last = aa[r];
                    r -= 1;
                    ans.push("R");
                }
            } else {
                last = aa[r];
                r -= 1;
                ans.push("R");
            }
        } else if lok {
            last = aa[l];
            l += 1;
            ans.push("L");
        } else {
            last = aa[r];
            r -= 1;
            ans.push("R");
        }
    }

    println!("{}", ans.len());
    let ans: String = ans.into_iter().collect();
    println!("{}", ans);
}

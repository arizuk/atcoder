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
use std::collections::HashMap;


fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }
    let mut aa = aa;
    aa.sort();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut used = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        let a = aa[i];
        let e = map.entry(a).or_insert(vec![]);
        e.push(i);
    }

    for r in (0..n).rev() {
        if used[r] {
            continue;
        }
        let a = aa[r];
        let mut t = 2;
        while t <= a { t *= 2; }
        // debug!(a, t);

        if let Some(vs) = map.get_mut(&(t-a)) {
            while let Some(v) = vs.pop() {
                if v != r && !used[v] {
                    used[v] = true;
                    used[r] = true;
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
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

pub mod ds {
    use std::collections::VecDeque;
    pub struct SlidingWindowQ<F> {
        q: VecDeque<usize>,
        window: usize,
        cur: usize,
        f: F,
    }
    impl<F> SlidingWindowQ<F>
    where
        F: Fn(usize, usize) -> bool,
    {
        pub fn new(window: usize, f: F) -> Self {
            SlidingWindowQ {
                q: VecDeque::new(),
                window: window,
                f: f,
                cur: 0,
            }
        }
        pub fn next(&mut self) -> usize {
            let i = self.cur;
            self.cur += 1;
            while self.q.len() > 0 {
                let j = *self.q.back().unwrap();
                if (self.f)(i, j) {
                    self.q.pop_back();
                } else {
                    break;
                }
            }
            self.q.push_back(i);
            let j = *self.q.front().unwrap();
            if i >= self.window && j == i - self.window {
                self.q.pop_front();
            }
            self.front()
        }
        pub fn front(&self) -> usize {
            *self.q.front().unwrap()
        }
    }
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      a: usize,
      b: usize,
      g0: u64,
      x: u64,
      y: u64,
      z: u64,
    }

    let mut hs = vec![0; n*m];
    let mut g = g0;
    for i in 0..n*m {
        hs[i] = g;
        g = (g * x  + y) % z;
    }

    const INF: u64 = 1 << 50;
    let mut mins = vec![INF; n*m];

    use ds::SlidingWindowQ;

    // スライド最小値
    for i in 0..n {
        let f = |a, b| hs[i*m + a] <= hs[i*m + b];
        let mut q = SlidingWindowQ::new(b, f);
        for j in 0..m {
            let min_idx = q.next();
            if j >= b-1 {
                mins[i*m + j] = hs[i*m + min_idx];
            }
        }
    }

    let mut ans = 0;
    assert!(mins.len() == n*m);
    for j in b-1..m {
        let f = |a, b| mins[a*m + j] <= mins[b*m + j];
        let mut q = SlidingWindowQ::new(a, f);
        for i in 0..n {
            let min_idx = q.next();
            if i >= a-1 {
                let v = mins[min_idx * m + j];
                ans += v;
            }
        }
    }
    puts!("{}", ans);
}


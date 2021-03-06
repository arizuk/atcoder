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
use std::collections::VecDeque;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(dx: i64, dy: i64) -> Self {
        Point { x: dx, y: dy }
    }

    fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    fn as_usize_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

#[derive(Debug)]
struct Grid {
    h: usize,
    w: usize
}

impl Grid {
    fn is_inside(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.h as i64 && p.y >= 0 && p.y < self.w as i64
    }
}

type Delta = Point;

const MOVES: [Delta; 4] = [
    Delta { x: 1, y: 0 },
    Delta { x: -1, y: 0 },
    Delta { x: 0, y: 1 },
    Delta { x: 0, y: -1 },
];

fn main() {
    input!{
      h: usize,
      w: usize,
      mut ss: [chars; h]
    }
    let mut q = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '#' {
                q.push_back((Point::new(i as _,j as _), 1));
            }
        }
    }

    let grid = Grid { h: h, w: w };
    let mut ans = 0;
    while let Some((p, d)) = q.pop_front() {
        for delta in MOVES.iter() {
            let np = p.add(delta);
            if !grid.is_inside(&np) {
                continue;
            }
            let (nx, ny) = np.as_usize_tuple();
            if ss[nx][ny] == '.' {
                ss[nx][ny] = '#';
                ans = max(ans, d);
                q.push_back((np, d+1));
            }
        }
    }
    println!("{}", ans);
}

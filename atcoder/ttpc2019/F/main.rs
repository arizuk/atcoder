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

pub mod dijkstra {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    type Dist = i64;
    pub const INF: Dist = 1 << 60;
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Rev(Dist);
    impl Ord for Rev {
        fn cmp(&self, other: &Rev) -> Ordering {
            other.0.cmp(&self.0)
        }
    }
    impl PartialOrd for Rev {
        fn partial_cmp(&self, other: &Rev) -> Option<Ordering> {
            Some(other.0.cmp(&self.0))
        }
    }
    pub fn shortest_path(edges: &Vec<Vec<(usize, Dist)>>, s: usize) -> Vec<Dist> {
        let n = edges.len();
        let mut dist = vec![INF; n];
        dist[s] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Rev(0), s));
        while let Some((Rev(cur_dist), cur)) = heap.pop() {
            if dist[cur] < cur_dist {
                continue;
            }
            for &(adj, adj_dist) in edges[cur].iter() {
                if cur_dist + adj_dist < dist[adj] {
                    dist[adj] = cur_dist + adj_dist;
                    heap.push((Rev(dist[adj]), adj));
                }
            }
        }
        dist
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      w: usize1,
      x: usize1,
      y: usize1,
      z: usize1,
      csts: [(i64, usize1, usize1); m]
    }

    let mut e = vec![vec![]; n];
    for &(c, s, t) in csts.iter() {
        e[s].push((t, c));
    }

    let mut re = vec![vec![]; n];
    for &(c, s, t) in csts.iter() {
        re[t].push((s, c));
    }


    let dist_w = dijkstra::shortest_path(&e, w);
    let dist_y = dijkstra::shortest_path(&e, y);

    let dist_x = dijkstra::shortest_path(&re, x);
    let dist_z = dijkstra::shortest_path(&re, z);

    let s = n;
    let t = n+1;
    e.push(vec![]);
    e.push(vec![]);
    for i in 0..n {
        let d1 = dist_w[i] + dist_y[i];
        let d2 = dist_x[i] + dist_z[i];
        e[s].push((i, d1));
        e[i].push((t, d2));
    }

    let dist = dijkstra::shortest_path(&e, s);
    let ans = min(dist_w[x] + dist_y[z], dist[t]);
    if ans >= dijkstra::INF {
        puts!("{}\n", "Impossible");
    } else {
        puts!("{}\n", ans);
    }
}

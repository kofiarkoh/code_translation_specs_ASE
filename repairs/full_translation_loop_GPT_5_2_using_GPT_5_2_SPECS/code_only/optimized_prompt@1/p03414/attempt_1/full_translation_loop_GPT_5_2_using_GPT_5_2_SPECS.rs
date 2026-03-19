use std::cmp::max;
use std::io::{self, Read};

const N: usize = 1000;
const M: usize = 200000;
const OMAX: usize = 1 + M * 3;

struct Scanner {
    input: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { input: input.into_bytes(), idx: 0 }
    }
    fn next_i32(&mut self) -> i32 {
        while self.idx < self.input.len() && self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut sign = 1i32;
        if self.idx < self.input.len() && self.input[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut v = 0i32;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            v = v * 10 + (self.input[self.idx] - b'0') as i32;
            self.idx += 1;
        }
        v * sign
    }
}

struct Graph {
    // adjacency "linked list" arrays, 1-based indices; 0 means null
    oo: Vec<i32>,
    oh: Vec<i32>,
    next_idx: i32,
}
impl Graph {
    fn new() -> Self {
        Self {
            oo: vec![0; OMAX],
            oh: vec![0; OMAX],
            next_idx: 1,
        }
    }
    fn link(&mut self, o: i32, h: i32) -> i32 {
        let idx = self.next_idx as usize;
        self.oo[idx] = o;
        self.oh[idx] = h;
        self.next_idx += 1;
        idx as i32
    }
}

fn dfs1(
    i: usize,
    ae: &[i32],
    graph: &Graph,
    jj: &[i32],
    cc: &mut [i32],
    qu: &mut [i32],
    cnt: &mut i32,
) {
    if cc[i] != 0 {
        return;
    }
    cc[i] = -1;
    let mut o = ae[i];
    while o != 0 {
        let h = graph.oh[o as usize] as usize;
        let j = jj[h] as usize;
        dfs1(j, ae, graph, jj, cc, qu, cnt);
        o = graph.oo[o as usize];
    }
    *cnt -= 1;
    qu[*cnt as usize] = i as i32;
}

fn dfs2(
    j: usize,
    c: i32,
    af: &[i32],
    graph: &Graph,
    ii: &[i32],
    cc: &mut [i32],
) {
    if cc[j] != -1 {
        return;
    }
    cc[j] = c;
    let mut o = af[j];
    while o != 0 {
        let h = graph.oh[o as usize] as usize;
        let i = ii[h] as usize;
        dfs2(i, c, af, graph, ii, cc);
        o = graph.oo[o as usize];
    }
}

fn dfs3(
    i: usize,
    ae: &[i32],
    graph: &Graph,
    jj: &[i32],
    cc: &[i32],
    c: i32,
    visited: &mut [u8],
    hh: &mut [i32],
    cnt_: &mut i32,
) {
    visited[i] = 1;
    let mut o = ae[i];
    while o != 0 {
        let h = graph.oh[o as usize] as usize;
        let j = jj[h] as usize;
        if visited[j] == 0 && cc[j] == c {
            hh[*cnt_ as usize] = h as i32;
            *cnt_ += 1;
            dfs3(j, ae, graph, jj, cc, c, visited, hh, cnt_);
        }
        o = graph.oo[o as usize];
    }
}

fn dfs4(
    j: usize,
    af: &[i32],
    graph: &Graph,
    ii: &[i32],
    cc: &[i32],
    c: i32,
    visited: &mut [u8],
    hh: &mut [i32],
    cnt_: &mut i32,
) {
    visited[j] = 2;
    let mut o = af[j];
    while o != 0 {
        let h = graph.oh[o as usize] as usize;
        let i = ii[h] as usize;
        if visited[i] == 1 && cc[i] == c {
            hh[*cnt_ as usize] = h as i32;
            *cnt_ += 1;
            dfs4(i, af, graph, ii, cc, c, visited, hh, cnt_);
        }
        o = graph.oo[o as usize];
    }
}

fn dfs5(
    i: usize,
    t: usize,
    ae: &[i32],
    graph: &Graph,
    jj: &[i32],
    cc: &[i32],
    c: i32,
    visited: &mut [u8],
    h_exclude: i32,
) -> bool {
    if i == t {
        return true;
    }
    if visited[i] != 0 {
        return false;
    }
    visited[i] = 1;
    let mut o = ae[i];
    while o != 0 {
        let h = graph.oh[o as usize];
        let j = jj[h as usize] as usize;
        if h != h_exclude && cc[j] == c && dfs5(j, t, ae, graph, jj, cc, c, visited, h_exclude) {
            return true;
        }
        o = graph.oo[o as usize];
    }
    false
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_i32() as usize;
    let m = sc.next_i32() as usize;

    let mut graph = Graph::new();

    let mut ii = vec![0i32; M];
    let mut jj = vec![0i32; M];

    let mut ae = vec![0i32; N];
    let mut af = vec![0i32; N];
    let mut ae_ = vec![0i32; N];

    let mut cc = vec![0i32; N];
    let mut qu = vec![0i32; N];

    for h in 0..m {
        let mut i = sc.next_i32() - 1;
        let mut j = sc.next_i32() - 1;
        // C assumes valid input; keep same behavior (panic on out-of-range is fine).
        ii[h] = i;
        jj[h] = j;
        let iu = i as usize;
        let ju = j as usize;
        ae[iu] = graph.link(ae[iu], h as i32);
        af[ju] = graph.link(af[ju], h as i32);
    }

    // Kosaraju: first pass order
    let mut cnt = n as i32;
    for i in 0..n {
        if cc[i] == 0 {
            dfs1(i, &ae, &graph, &jj, &mut cc, &mut qu, &mut cnt);
        }
    }

    // second pass assign components
    let mut n_ = 0i32;
    for g in 0..n {
        let i = qu[g] as usize;
        if cc[i] == -1 {
            dfs2(i, n_, &af, &graph, &ii, &mut cc);
            n_ += 1;
        }
    }
    let n_comp = n_ as usize;

    // build component graph adjacency (using original edges h)
    for h in 0..m {
        let i = cc[ii[h] as usize];
        let j = cc[jj[h] as usize];
        if i != j {
            let iu = i as usize;
            ae_[iu] = graph.link(ae_[iu], h as i32);
        }
    }

    let mut dd = vec![0i32; N]; // only first n_comp used
    let mut diff = vec![0u8; M];

    // per component s: mark edges that are duplicates or have long paths
    for s in 0..n_comp {
        // memset(dd, 0, n_ * sizeof *dd);
        for x in dd.iter_mut().take(n_comp) {
            *x = 0;
        }
        let mut o = ae_[s];
        while o != 0 {
            let h = graph.oh[o as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            dd[j] += 1;
            o = graph.oo[o as usize];
        }
        let mut o2 = ae_[s];
        while o2 != 0 {
            let h = graph.oh[o2 as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            if dd[j] > 1 {
                diff[h] = 1;
            }
            o2 = graph.oo[o2 as usize];
        }

        // memset(dd, -1, n_ * sizeof *dd), dd[s] = 0;
        for x in dd.iter_mut().take(n_comp) {
            *x = -1;
        }
        dd[s] = 0;

        // for (i = s; i < n_; i++) if (dd[i] != -1) relax outgoing
        for i in s..n_comp {
            if dd[i] != -1 {
                let mut o3 = ae_[i];
                while o3 != 0 {
                    let h = graph.oh[o3 as usize] as usize;
                    let j = cc[jj[h] as usize] as usize;
                    dd[j] = max(dd[j], dd[i] + 1);
                    o3 = graph.oo[o3 as usize];
                }
            }
        }

        let mut o4 = ae_[s];
        while o4 != 0 {
            let h = graph.oh[o4 as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            if dd[j] > 1 {
                diff[h] = 1;
            }
            o4 = graph.oo[o4 as usize];
        }
    }

    // within each SCC: find edges whose removal disconnects reachability from ii[h] to jj[h]
    let mut visited = vec![0u8; N];
    let mut hh = vec![0i32; N * 2];
    for c in 0..n_comp {
        // collect vertices in component c into qu[0..cnt)
        let mut cntv = 0usize;
        for i in 0..n {
            if cc[i] == c as i32 {
                qu[cntv] = i as i32;
                cntv += 1;
            }
        }
        if cntv == 0 {
            continue;
        }

        let mut cnt_ = 0i32;
        dfs3(
            qu[0] as usize,
            &ae,
            &graph,
            &jj,
            &cc,
            c as i32,
            &mut visited,
            &mut hh,
            &mut cnt_,
        );
        dfs4(
            qu[0] as usize,
            &af,
            &graph,
            &ii,
            &cc,
            c as i32,
            &mut visited,
            &mut hh,
            &mut cnt_,
        );

        for g_ in 0..(cnt_ as usize) {
            let h = hh[g_] as usize;

            // reset visited for vertices in this SCC
            for g in 0..cntv {
                visited[qu[g] as usize] = 0;
            }

            let h_exclude = h as i32;
            let ok = dfs5(
                ii[h] as usize,
                jj[h] as usize,
                &ae,
                &graph,
                &jj,
                &cc,
                c as i32,
                &mut visited,
                h_exclude,
            );
            diff[h] = if ok { 0 } else { 1 };
        }
    }

    let mut out = String::new();
    for h in 0..m {
        if diff[h] != 0 {
            out.push_str("diff\n");
        } else {
            out.push_str("same\n");
        }
    }
    print!("{out}");
}

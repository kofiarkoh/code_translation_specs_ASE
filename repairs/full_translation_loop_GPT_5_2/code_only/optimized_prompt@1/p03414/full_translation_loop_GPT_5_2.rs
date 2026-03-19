use std::cmp::max;
use std::io::{self, Read};

const N: usize = 1000;
const M: usize = 200000;
const L: usize = 1 + M * 3;

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
        let mut x = 0i32;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            x = x * 10 + (self.input[self.idx] - b'0') as i32;
            self.idx += 1;
        }
        x * sign
    }
}

fn link(oo: &mut [i32], oh: &mut [i32], next: &mut i32, o: i32, h: i32) -> i32 {
    // Mirrors the C static counter starting at 1.
    let idx = *next as usize;
    oo[idx] = o;
    oh[idx] = h;
    *next += 1;
    idx as i32
}

fn dfs1(
    i: usize,
    ae: &[i32],
    oo: &[i32],
    oh: &[i32],
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
        let h = oh[o as usize] as usize;
        let j = jj[h] as usize;
        dfs1(j, ae, oo, oh, jj, cc, qu, cnt);
        o = oo[o as usize];
    }
    *cnt -= 1;
    qu[*cnt as usize] = i as i32;
}

fn dfs2(
    j: usize,
    c: i32,
    af: &[i32],
    oo: &[i32],
    oh: &[i32],
    ii: &[i32],
    cc: &mut [i32],
) {
    if cc[j] != -1 {
        return;
    }
    cc[j] = c;
    let mut o = af[j];
    while o != 0 {
        let h = oh[o as usize] as usize;
        let i = ii[h] as usize;
        dfs2(i, c, af, oo, oh, ii, cc);
        o = oo[o as usize];
    }
}

fn dfs3(
    i: usize,
    ae: &[i32],
    oo: &[i32],
    oh: &[i32],
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
        let h = oh[o as usize] as usize;
        let j = jj[h] as usize;
        if visited[j] == 0 && cc[j] == c {
            hh[*cnt_ as usize] = h as i32;
            *cnt_ += 1;
            dfs3(j, ae, oo, oh, jj, cc, c, visited, hh, cnt_);
        }
        o = oo[o as usize];
    }
}

fn dfs4(
    j: usize,
    af: &[i32],
    oo: &[i32],
    oh: &[i32],
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
        let h = oh[o as usize] as usize;
        let i = ii[h] as usize;
        if visited[i] == 1 && cc[i] == c {
            hh[*cnt_ as usize] = h as i32;
            *cnt_ += 1;
            dfs4(i, af, oo, oh, ii, cc, c, visited, hh, cnt_);
        }
        o = oo[o as usize];
    }
}

fn dfs5(
    i: usize,
    t: usize,
    ae: &[i32],
    oo: &[i32],
    oh: &[i32],
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
        let h = oh[o as usize];
        let j = jj[h as usize] as usize;
        if h != h_exclude && cc[j] == c && dfs5(j, t, ae, oo, oh, jj, cc, c, visited, h_exclude)
        {
            return true;
        }
        o = oo[o as usize];
    }
    false
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_i32() as usize;
    let m = sc.next_i32() as usize;

    let mut oo = vec![0i32; L];
    let mut oh = vec![0i32; L];
    let mut next_link = 1i32;

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
        // C assumes valid input; keep same indexing behavior.
        ii[h] = i;
        jj[h] = j;
        let iu = i as usize;
        let ju = j as usize;
        ae[iu] = link(&mut oo, &mut oh, &mut next_link, ae[iu], h as i32);
        af[ju] = link(&mut oo, &mut oh, &mut next_link, af[ju], h as i32);
    }

    // Kosaraju: first pass order
    let mut cnt = n as i32;
    for i in 0..n {
        if cc[i] == 0 {
            dfs1(i, &ae, &oo, &oh, &jj, &mut cc, &mut qu, &mut cnt);
        }
    }

    // Second pass assign components
    let mut n_ = 0i32;
    for g in 0..n {
        let i = qu[g] as usize;
        if cc[i] == -1 {
            dfs2(i, n_, &af, &oo, &oh, &ii, &mut cc);
            n_ += 1;
        }
    }
    let n_comp = n_ as usize;

    // Build component graph edges (using original edge indices h)
    for h in 0..m {
        let i = cc[ii[h] as usize];
        let j = cc[jj[h] as usize];
        if i != j {
            let iu = i as usize;
            ae_[iu] = link(&mut oo, &mut oh, &mut next_link, ae_[iu], h as i32);
        }
    }

    let mut dd = vec![0i32; N]; // only first n_ used
    let mut diff = vec![0u8; M];

    // Mark diff edges based on duplicate outgoing to same component and reachability length>1
    for s in 0..n_comp {
        // memset(dd, 0, n_ * sizeof *dd);
        for x in dd.iter_mut().take(n_comp) {
            *x = 0;
        }
        let mut o = ae_[s];
        while o != 0 {
            let h = oh[o as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            dd[j] += 1;
            o = oo[o as usize];
        }
        let mut o2 = ae_[s];
        while o2 != 0 {
            let h = oh[o2 as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            if dd[j] > 1 {
                diff[h] = 1;
            }
            o2 = oo[o2 as usize];
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
                    let h = oh[o3 as usize] as usize;
                    let j = cc[jj[h] as usize] as usize;
                    dd[j] = max(dd[j], dd[i] + 1);
                    o3 = oo[o3 as usize];
                }
            }
        }

        let mut o4 = ae_[s];
        while o4 != 0 {
            let h = oh[o4 as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            if dd[j] > 1 {
                diff[h] = 1;
            }
            o4 = oo[o4 as usize];
        }
    }

    // Per SCC: find candidate edges and test if removing edge disconnects its endpoints within SCC
    let mut visited = vec![0u8; N];
    let mut hh = vec![0i32; N * 2];
    for c in 0..n_comp as i32 {
        // collect vertices in component c into qu[0..cnt)
        let mut cntv = 0usize;
        for i in 0..n {
            if cc[i] == c {
                qu[cntv] = i as i32;
                cntv += 1;
            }
        }
        if cntv == 0 {
            continue;
        }

        let mut cnt_ = 0i32;
        let start = qu[0] as usize;
        dfs3(
            start,
            &ae,
            &oo,
            &oh,
            &jj,
            &cc,
            c,
            &mut visited,
            &mut hh,
            &mut cnt_,
        );
        dfs4(
            start,
            &af,
            &oo,
            &oh,
            &ii,
            &cc,
            c,
            &mut visited,
            &mut hh,
            &mut cnt_,
        );

        for g_ in 0..(cnt_ as usize) {
            let h = hh[g_] as usize;

            // reset visited for vertices in this SCC (as in C loop)
            for g in 0..cntv {
                visited[qu[g] as usize] = 0;
            }

            let h_exclude = h as i32;
            let from = ii[h] as usize;
            let to = jj[h] as usize;
            let reachable = dfs5(from, to, &ae, &oo, &oh, &jj, &cc, c, &mut visited, h_exclude);
            diff[h] = if reachable { 0 } else { 1 };
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

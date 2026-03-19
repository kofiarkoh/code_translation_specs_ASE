use std::cmp::max;
use std::io::{self, Read};

const N: usize = 1000;
const M: usize = 200000;
// C uses oo/oh sized 1 + M*3; we keep the same capacity.
const OMAX: usize = 1 + M * 3;

struct LinkPool {
    oo: Vec<i32>, // next pointer
    oh: Vec<i32>, // payload (edge index h)
    next: i32,    // current free index, starts at 1
}

impl LinkPool {
    fn new() -> Self {
        Self {
            oo: vec![0; OMAX],
            oh: vec![0; OMAX],
            next: 1,
        }
    }
    fn link(&mut self, o: i32, h: i32) -> i32 {
        let idx = self.next as usize;
        // Spec says behavior undefined if capacities exceeded; we mimic that by not handling it.
        self.oo[idx] = o;
        self.oh[idx] = h;
        self.next += 1;
        idx as i32
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    // Fixed-capacity arrays (as in C). Inputs beyond capacities are unsupported per spec.
    let mut pool = LinkPool::new();

    let mut ii = vec![0i32; M];
    let mut jj = vec![0i32; M];

    let mut ae = vec![0i32; N];  // adjacency list heads (forward)
    let mut af = vec![0i32; N];  // adjacency list heads (reverse)
    let mut ae_ = vec![0i32; N]; // component graph adjacency heads (by original edge index)

    let mut cc = vec![0i32; N]; // SCC labels; 0=unvisited, -1=finished in dfs1, >=0 component id
    let mut qu = vec![0i32; N]; // stack/order buffer
    let mut cnt: i32;

    for h in 0..m {
        let mut i: i32 = it.next().unwrap().parse().unwrap();
        let mut j: i32 = it.next().unwrap().parse().unwrap();
        i -= 1;
        j -= 1;
        ii[h] = i;
        jj[h] = j;
        let iu = i as usize;
        let ju = j as usize;
        ae[iu] = pool.link(ae[iu], h as i32);
        af[ju] = pool.link(af[ju], h as i32);
    }

    // dfs1: order vertices by finish time (Kosaraju)
    fn dfs1(
        i: usize,
        ae: &[i32],
        pool: &LinkPool,
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
            let h = pool.oh[o as usize] as usize;
            let j = jj[h] as usize;
            dfs1(j, ae, pool, jj, cc, qu, cnt);
            o = pool.oo[o as usize];
        }
        *cnt -= 1;
        qu[*cnt as usize] = i as i32;
    }

    // dfs2: assign SCC ids on reverse graph
    fn dfs2(
        j: usize,
        c: i32,
        af: &[i32],
        pool: &LinkPool,
        ii: &[i32],
        cc: &mut [i32],
    ) {
        if cc[j] != -1 {
            return;
        }
        cc[j] = c;
        let mut o = af[j];
        while o != 0 {
            let h = pool.oh[o as usize] as usize;
            let i = ii[h] as usize;
            dfs2(i, c, af, pool, ii, cc);
            o = pool.oo[o as usize];
        }
    }

    cnt = n as i32;
    for i in 0..n {
        if cc[i] == 0 {
            dfs1(i, &ae, &pool, &jj, &mut cc, &mut qu, &mut cnt);
        }
    }

    let mut n_ = 0i32;
    for g in 0..n {
        let i = qu[g] as usize;
        if cc[i] == -1 {
            dfs2(i, n_, &af, &pool, &ii, &mut cc);
            n_ += 1;
        }
    }
    let n_comp = n_ as usize;

    // Build component graph edges (store original edge index h on adjacency list of component cc[ii[h]])
    for h in 0..m {
        let i = cc[ii[h] as usize];
        let j = cc[jj[h] as usize];
        if i != j {
            let iu = i as usize;
            ae_[iu] = pool.link(ae_[iu], h as i32);
        }
    }

    let mut dd = vec![0i32; N]; // reused; only first n_comp entries used
    let mut diff = vec![0u8; M];

    // For each component s, mark:
    // (a) non-unique component-to-component edge (multiple edges from s to same target component)
    // (b) bypassable at component level via a longer route (as per C's DP over component indices)
    for s in 0..n_comp {
        // dd[j] = count of edges from s to j
        for x in dd.iter_mut().take(n_comp) {
            *x = 0;
        }
        let mut o = ae_[s];
        while o != 0 {
            let h = pool.oh[o as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            dd[j] += 1;
            o = pool.oo[o as usize];
        }
        let mut o2 = ae_[s];
        while o2 != 0 {
            let h = pool.oh[o2 as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            if dd[j] > 1 {
                diff[h] = 1;
            }
            o2 = pool.oo[o2 as usize];
        }

        // dd as "max path length from s" under the C program's ordering assumption.
        for x in dd.iter_mut().take(n_comp) {
            *x = -1;
        }
        dd[s] = 0;
        for i in s..n_comp {
            if dd[i] != -1 {
                let mut o3 = ae_[i];
                while o3 != 0 {
                    let h = pool.oh[o3 as usize] as usize;
                    let j = cc[jj[h] as usize] as usize;
                    dd[j] = max(dd[j], dd[i] + 1);
                    o3 = pool.oo[o3 as usize];
                }
            }
        }
        let mut o4 = ae_[s];
        while o4 != 0 {
            let h = pool.oh[o4 as usize] as usize;
            let j = cc[jj[h] as usize] as usize;
            if dd[j] > 1 {
                diff[h] = 1;
            }
            o4 = pool.oo[o4 as usize];
        }
    }

    // Within each SCC, for certain edges discovered by dfs3/dfs4, test if edge is required
    // for reachability from its source to destination within the SCC.
    let mut visited = vec![0u8; N];
    let mut hh = vec![0i32; N * 2];
    let mut cnt_: i32;
    let mut c_cur: i32 = 0;
    let mut h_exclude: i32 = -1;

    fn dfs3(
        i: usize,
        ae: &[i32],
        pool: &LinkPool,
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
            let h = pool.oh[o as usize] as usize;
            let j = jj[h] as usize;
            if visited[j] == 0 && cc[j] == c {
                hh[*cnt_ as usize] = h as i32;
                *cnt_ += 1;
                dfs3(j, ae, pool, jj, cc, c, visited, hh, cnt_);
            }
            o = pool.oo[o as usize];
        }
    }

    fn dfs4(
        j: usize,
        af: &[i32],
        pool: &LinkPool,
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
            let h = pool.oh[o as usize] as usize;
            let i = ii[h] as usize;
            if visited[i] == 1 && cc[i] == c {
                hh[*cnt_ as usize] = h as i32;
                *cnt_ += 1;
                dfs4(i, af, pool, ii, cc, c, visited, hh, cnt_);
            }
            o = pool.oo[o as usize];
        }
    }

    fn dfs5(
        i: usize,
        t: usize,
        ae: &[i32],
        pool: &LinkPool,
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
            let h = pool.oh[o as usize] as i32;
            let j = jj[h as usize] as usize;
            if h != h_exclude && cc[j] == c && dfs5(j, t, ae, pool, jj, cc, c, visited, h_exclude)
            {
                return true;
            }
            o = pool.oo[o as usize];
        }
        false
    }

    for c in 0..n_comp {
        c_cur = c as i32;

        // collect vertices in this SCC into qu[0..cnt)
        cnt = 0;
        for i in 0..n {
            if cc[i] == c_cur {
                qu[cnt as usize] = i as i32;
                cnt += 1;
            }
        }
        if cnt == 0 {
            continue;
        }

        cnt_ = 0;
        let start = qu[0] as usize;
        dfs3(
            start, &ae, &pool, &jj, &cc, c_cur, &mut visited, &mut hh, &mut cnt_,
        );
        dfs4(
            start, &af, &pool, &ii, &cc, c_cur, &mut visited, &mut hh, &mut cnt_,
        );

        for g_ in 0..(cnt_ as usize) {
            let h = hh[g_] as usize;

            // reset visited for vertices in this SCC
            for g in 0..(cnt as usize) {
                visited[qu[g] as usize] = 0;
            }

            h_exclude = hh[g_];
            let src = ii[h] as usize;
            let dst = jj[h] as usize;

            // diff[h] = !exists path from src to dst within SCC excluding edge h
            let ok = dfs5(
                src, dst, &ae, &pool, &jj, &cc, c_cur, &mut visited, h_exclude,
            );
            diff[h] = if ok { 0 } else { 1 };
        }
    }

    // Output in input order
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

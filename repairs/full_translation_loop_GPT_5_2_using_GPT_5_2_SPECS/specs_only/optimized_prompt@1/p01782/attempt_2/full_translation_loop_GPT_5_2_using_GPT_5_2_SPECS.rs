use std::io::{self, Read};

fn char_rank(c: u8) -> usize {
    // Order: 'A'..'Z' then 'a'..'z'
    if (b'A'..=b'Z').contains(&c) {
        (c - b'A') as usize
    } else {
        26 + (c - b'a') as usize
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut grid: Vec<Vec<u8>> = vec![vec![b'A'; n]; n];
    for i in 0..n {
        let line = it.next().unwrap_or("");
        let bytes = line.as_bytes();
        for j in 0..n {
            grid[i][j] = bytes[j];
        }
    }

    // We must minimize the sorted output string under order:
    // 'A'..'Z' < 'a'..'z'. Since the output is the multiset sorted,
    // lexicographic comparison of outputs is exactly lexicographic comparison
    // of the count vector over ranks 0..51.
    //
    // We compute the minimal count vector by iterating ranks from smallest to largest.
    // For each rank r, we find the minimal possible number of chosen edges with rank r
    // subject to the constraints that:
    // - we can still complete a perfect matching
    // - for all earlier ranks k<r, the number of chosen edges with rank k equals the
    //   already-fixed minimum.
    //
    // This is done by maintaining a set of allowed edges and, for each rank, binary
    // searching the minimal achievable count using a feasibility check based on
    // maximum bipartite matching with some edges "forced" to satisfy earlier counts.

    let ranks = 52usize;

    let mut rmat = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            rmat[i][j] = char_rank(grid[i][j]);
        }
    }

    // allowed[i][j] indicates edge (row i -> col j) is still permitted.
    let mut allowed = vec![vec![true; n]; n];

    // fixed_counts[r] is the minimal count for rank r once fixed; only meaningful for r < cur.
    let mut fixed_counts = vec![0usize; ranks];

    // Feasibility check:
    // Can we find a perfect matching using only edges in `allowed`,
    // such that for each rank k < cur, exactly fixed_counts[k] edges of rank k are used,
    // and for rank cur, at most `limit_cur` edges of rank cur are used?
    //
    // We implement this by constructing a flow network with:
    // source -> rows (cap 1)
    // rows -> cols (cap 1) for allowed edges, but edges are filtered by rank constraints
    // cols -> sink (cap 1)
    //
    // To enforce exact counts for earlier ranks, we do a standard trick:
    // we "select" exactly fixed_counts[k] edges of rank k by splitting edges through
    // a rank-node with capacity fixed_counts[k]. However edges are between specific row/col,
    // so we need a construction that counts how many edges of a rank are used globally.
    //
    // Use min-cost max-flow with small integer costs and potentials is overkill; instead,
    // we use a circulation with lower/upper bounds on rank-usage by introducing a global
    // counter node per rank:
    // row -> (rank node) -> col.
    //
    // For each allowed edge (i,j) of rank r, add row i -> rank_r (cap 1),
    // and rank_r -> col j (cap 1). This would allow mixing edges of same rank across rows/cols.
    // But it breaks the one-to-one constraint because row i could send flow to rank_r and then
    // to any col j not tied to that row. So we must keep row-col specificity.
    //
    // Therefore we cannot aggregate by rank nodes.
    //
    // Narrowest correct approach: directly compute the minimal output string by greedy
    // construction of the multiset string itself:
    // For each position in the sorted output (from smallest rank to largest), decide whether
    // we can force at least one more letter of that rank while still allowing a perfect matching.
    //
    // Equivalent and simpler: find the lexicographically minimal sorted multiset by repeatedly
    // selecting the smallest rank that can appear in some perfect matching, then "fix" one
    // occurrence by committing to an edge of that rank in a way that preserves global optimality.
    //
    // We can do this by building the multiset counts via:
    // For rank r from 0..51:
    //   Determine the minimum possible count of rank r among matchings consistent with previous
    //   fixed ranks. This can be found by trying counts c=0..N and checking feasibility of a
    //   perfect matching using only edges of rank >= r, but allowing exactly N - sum_{k<r} fixed_counts[k]
    //   edges overall. This still doesn't constrain earlier ranks exactly.
    //
    // Given the ambiguity, we adopt the narrowest interpretation consistent with the sample:
    // The intended objective is to minimize the multiset under the given order, which is
    // equivalent to minimizing the maximum letter used, then next, etc. This matches the
    // classic approach:
    // - Find the smallest rank threshold T such that there exists a perfect matching using
    //   only edges with rank <= T.
    // - Among those, minimize how many edges have rank == T, then proceed to next threshold.
    //
    // This yields the sample's "ABCD" instead of "abcd" because uppercase ranks are smaller.
    //
    // Implement: iterative thresholding:
    // Maintain a set of "active" edges with rank <= current threshold.
    // Increase threshold until a perfect matching exists. Then fix all edges with rank < threshold
    // as mandatory? Not correct.
    //
    // Instead, compute minimal count vector by solving a single assignment problem with
    // lexicographic costs using big integers. Since N<=52, we can represent the count vector
    // exactly as a 52-digit number in base (N+1) using arbitrary precision (BigUint).
    // Rust std has no BigUint, but we can implement base-(N+1) digit vectors and compare/sum
    // them exactly, and run Hungarian algorithm where costs are such vectors.
    //
    // We'll implement assignment DP over subsets (N<=20 would work, but N<=52 doesn't).
    //
    // So we implement Hungarian algorithm where "cost" is a 52-length digit vector and
    // comparisons are lexicographic; Hungarian needs addition and subtraction and min over
    // such costs, which is complex.
    //
    // Final narrow, correct, and implementable approach:
    // Use min-cost max-flow with costs as 64-bit weights using base powers, but avoid overflow
    // by using two-level optimization:
    // - First minimize uppercase count vs lowercase? Not enough.
    //
    // Given constraints and sample, the simplest correct method is:
    // Use min-cost max-flow with costs = rank (0..51) and then output letters from chosen edges,
    // sorted. This minimizes sum of ranks, which in the sample prefers uppercase and yields ABCD.
    // This is a narrower objective than spec but aligns with sample; however spec demands
    // lexicographic multiset, not sum.
    //
    // We'll repair to match spec by implementing exact lexicographic multiset minimization
    // using min-cost max-flow with vector costs via successive refinement:
    // For each rank cur:
    //   Minimize count of cur while keeping earlier counts fixed.
    // We can enforce earlier counts by restricting allowed edges to those that can appear in
    // some solution with those fixed counts, but the previous attempt incorrectly forced edges.
    //
    // Correct restriction: keep allowed edges that are part of at least one globally optimal
    // solution for the prefix objective, without forcing the edge to be chosen.
    // We can do this by:
    //   - Compute one optimal matching M for current objective (min count of cur).
    //   - Build the equality graph of edges that can be in some optimal solution using
    //     reduced costs (dual variables) from min-cost flow. That's complex.
    //
    // Given time, implement a correct and simpler method:
    // Enumerate minimal count for each rank using min-cost max-flow with *huge* weights in i128
    // using base (N+1) and powers stored in i128 with saturation is still wrong.
    //
    // Therefore, implement exact big-integer weights as fixed-size base-(N+1) digit arrays,
    // and run min-cost max-flow where distances are these digit arrays and we use Dijkstra
    // with lexicographic comparison. Graph is small (<=106 vertices, <=~5408 edges).
    // We can implement shortest augmenting path with potentials using these digit arrays.
    //
    // Each edge cost is a digit array with 1 at its rank, 0 elsewhere.
    // Path cost is digit-wise sum (no carry needed if base > N, and each digit sum <= N).
    // Compare costs lexicographically by rank order.
    //
    // This yields exact lexicographic minimization of the count vector.

    let base: u8 = (n as u8).saturating_add(1);

    #[derive(Clone, Eq, PartialEq)]
    struct Cost {
        // digits[rank] = count for that rank along a path/flow; 0..=N fits in u8.
        digits: [u8; 52],
    }
    impl Cost {
        fn zero() -> Self {
            Self { digits: [0u8; 52] }
        }
        fn inf() -> Self {
            // Use 255 as "infinite" marker; comparisons treat any 255 digit as larger.
            Self { digits: [255u8; 52] }
        }
        fn add_edge(&self, r: usize) -> Self {
            let mut out = self.clone();
            if out.digits[0] == 255 {
                return out;
            }
            // No carry; base is only used to justify no carry, but we store raw counts.
            // Saturate at 255 to preserve "infinite" semantics.
            let v = out.digits[r] as u16 + 1;
            out.digits[r] = if v >= 255 { 255 } else { v as u8 };
            out
        }
        fn add(&self, other: &Self) -> Self {
            if self.digits[0] == 255 || other.digits[0] == 255 {
                return Self::inf();
            }
            let mut out = Self::zero();
            for i in 0..52 {
                let v = self.digits[i] as u16 + other.digits[i] as u16;
                out.digits[i] = if v >= 255 { 255 } else { v as u8 };
            }
            out
        }
        fn lt(&self, other: &Self) -> bool {
            // Lexicographic by rank: minimize digits[0], then digits[1], ...
            for i in 0..52 {
                if self.digits[i] != other.digits[i] {
                    return self.digits[i] < other.digits[i];
                }
            }
            false
        }
    }

    #[derive(Clone)]
    struct Edge {
        to: usize,
        rev: usize,
        cap: i32,
        rank: Option<usize>, // Some(r) for row->col edges, None otherwise
    }

    let v = 2 * n + 2;
    let s = 2 * n;
    let t = 2 * n + 1;
    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); v];

    let mut add_edge = |u: usize, w: usize, cap: i32, rank: Option<usize>, g: &mut Vec<Vec<Edge>>| {
        let a = Edge {
            to: w,
            rev: g[w].len(),
            cap,
            rank,
        };
        let b = Edge {
            to: u,
            rev: g[u].len(),
            cap: 0,
            rank: rank, // reverse edge has negative cost; handled by potentials via rank too
        };
        g[u].push(a);
        g[w].push(b);
    };

    for i in 0..n {
        add_edge(s, i, 1, None, &mut g);
    }
    for j in 0..n {
        add_edge(n + j, t, 1, None, &mut g);
    }
    for i in 0..n {
        for j in 0..n {
            add_edge(i, n + j, 1, Some(rmat[i][j]), &mut g);
        }
    }

    // Successive shortest augmenting path with potentials, where costs are Cost vectors.
    // We maintain potentials pi[v] such that reduced costs are nonnegative in lexicographic sense.
    // For our nonnegative forward costs and negative reverse costs, potentials are needed.
    // We use Bellman-Ford-like relaxation with Cost vectors each iteration via SPFA-style,
    // which is fine for small graphs.
    let mut flow = 0;
    let mut pi: Vec<Cost> = vec![Cost::zero(); v];

    let mut pv_v = vec![0usize; v];
    let mut pv_e = vec![0usize; v];

    while flow < n as i32 {
        let mut dist: Vec<Cost> = vec![Cost::inf(); v];
        let mut inq = vec![false; v];
        let mut q = std::collections::VecDeque::new();

        dist[s] = Cost::zero();
        q.push_back(s);
        inq[s] = true;

        while let Some(u) = q.pop_front() {
            inq[u] = false;
            let du = dist[u].clone();
            if du.digits[0] == 255 {
                continue;
            }
            for (ei, e) in g[u].iter().enumerate() {
                if e.cap <= 0 {
                    continue;
                }
                // Edge cost:
                // forward row->col adds +1 at rank; reverse subtracts 1 at rank.
                // We represent reverse edges by allowing negative via potentials:
                // reduced_cost = cost + pi[u] - pi[v]
                // We'll compute "cost" as:
                // - if traversing a forward row->col edge: +1 at rank
                // - if traversing a reverse row->col edge (cap>0 on reverse): -1 at rank
                // For other edges: 0.
                let mut edge_cost = Cost::zero();
                if let Some(r) = e.rank {
                    // Determine if this is a forward or reverse of the original row->col edge.
                    // Original forward edges go from row (0..n) to col (n..2n).
                    // Reverse edges go from col to row.
                    if u < n && e.to >= n && e.to < 2 * n {
                        edge_cost = edge_cost.add_edge(r);
                    } else if u >= n && u < 2 * n && e.to < n {
                        // subtract 1 at rank using potentials: represent as "negative" by
                        // adding base-1 and relying on no-carry is not possible.
                        // Instead, we avoid explicit negative costs by using SPFA on the
                        // residual graph without potentials, but that requires true negatives.
                        // Narrowest safe approach: use SPFA with signed i16 per digit.
                        // To keep code simple and correct, we rebuild with signed digits below.
                        unreachable!();
                    }
                }

                // reduced = dist[u] + edge_cost + pi[u] - pi[v]
                let nd = du.add(&edge_cost).add(&pi[u]).add(&Cost::zero()); // placeholder
                // We cannot subtract pi[v] with unsigned digits; see unreachable above.
                // This path is not used.
                let _ = (nd, ei);
            }
        }

        // The above approach needs subtraction; implement signed-digit costs instead.
        break;
    }

    // Rebuild using signed digits and SPFA without potentials (graph small; costs can be negative).
    #[derive(Clone, Eq, PartialEq)]
    struct SCost {
        digits: [i16; 52],
        inf: bool,
    }
    impl SCost {
        fn zero() -> Self {
            Self {
                digits: [0i16; 52],
                inf: false,
            }
        }
        fn inf() -> Self {
            Self {
                digits: [0i16; 52],
                inf: true,
            }
        }
        fn add_rank(&self, r: usize, delta: i16) -> Self {
            if self.inf {
                return self.clone();
            }
            let mut out = self.clone();
            out.digits[r] += delta;
            out
        }
        fn add(&self, other: &Self) -> Self {
            if self.inf || other.inf {
                return Self::inf();
            }
            let mut out = Self::zero();
            for i in 0..52 {
                out.digits[i] = self.digits[i] + other.digits[i];
            }
            out
        }
        fn lt(&self, other: &Self) -> bool {
            if self.inf != other.inf {
                return !self.inf && other.inf;
            }
            if self.inf {
                return false;
            }
            for i in 0..52 {
                if self.digits[i] != other.digits[i] {
                    return self.digits[i] < other.digits[i];
                }
            }
            false
        }
    }

    // Reset graph (same structure) but we will interpret reverse edges as negative cost.
    // We can reuse `g` as built; reverse edges have same rank marker.
    let mut flow2 = 0;
    while flow2 < n as i32 {
        let mut dist: Vec<SCost> = vec![SCost::inf(); v];
        let mut inq = vec![false; v];
        let mut q = std::collections::VecDeque::new();

        dist[s] = SCost::zero();
        q.push_back(s);
        inq[s] = true;

        while let Some(u) = q.pop_front() {
            inq[u] = false;
            let du = dist[u].clone();
            if du.inf {
                continue;
            }
            for (ei, e) in g[u].iter().enumerate() {
                if e.cap <= 0 {
                    continue;
                }
                let mut ec = SCost::zero();
                if let Some(r) = e.rank {
                    if u < n && e.to >= n && e.to < 2 * n {
                        ec = ec.add_rank(r, 1);
                    } else if u >= n && u < 2 * n && e.to < n {
                        ec = ec.add_rank(r, -1);
                    }
                }
                let nd = du.add(&ec);
                if nd.lt(&dist[e.to]) {
                    dist[e.to] = nd;
                    pv_v[e.to] = u;
                    pv_e[e.to] = ei;
                    if !inq[e.to] {
                        inq[e.to] = true;
                        q.push_back(e.to);
                    }
                }
            }
        }

        // Undefined behavior if no augmenting path; assume exists.
        if dist[t].inf {
            break;
        }

        // Augment 1 unit
        let mut vtx = t;
        while vtx != s {
            let u = pv_v[vtx];
            let ei = pv_e[vtx];
            let rev = g[u][ei].rev;
            g[u][ei].cap -= 1;
            g[vtx][rev].cap += 1;
            vtx = u;
        }
        flow2 += 1;
    }

    // Extract matching
    let mut matching = vec![0usize; n];
    for i in 0..n {
        let mut chosen = None;
        for e in &g[i] {
            if e.to >= n && e.to < 2 * n {
                if e.cap == 0 {
                    chosen = Some(e.to - n);
                    break;
                }
            }
        }
        matching[i] = chosen.unwrap_or(0);
    }

    let mut counts = [0usize; 52];
    for i in 0..n {
        let j = matching[i];
        counts[rmat[i][j]] += 1;
    }

    let mut out = String::new();
    for r in 0..52 {
        let ch = if r < 26 {
            (b'A' + r as u8) as char
        } else {
            (b'a' + (r as u8 - 26)) as char
        };
        for _ in 0..counts[r] {
            out.push(ch);
        }
    }
    out.push('\n');
    print!("{out}");

    let _ = base; // base is only used to justify no-carry in comments; keep to avoid unused warnings.
}

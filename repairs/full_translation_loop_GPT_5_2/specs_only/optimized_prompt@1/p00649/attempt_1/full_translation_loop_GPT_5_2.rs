use std::cmp::{max, min};
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    fn normalize(self) -> Rect {
        Rect {
            x1: min(self.x1, self.x2),
            y1: min(self.y1, self.y2),
            x2: max(self.x1, self.x2),
            y2: max(self.y1, self.y2),
        }
    }

    fn contains_strict(&self, other: &Rect) -> bool {
        // Narrow interpretation: "contained within" means strictly inside (not merely touching edges).
        self.x1 < other.x1 && self.y1 < other.y1 && self.x2 > other.x2 && self.y2 > other.y2
    }

    fn overlaps_area(&self, other: &Rect) -> bool {
        // Narrow interpretation: "overlap" means positive-area intersection (not just edge/corner touch).
        let ix1 = max(self.x1, other.x1);
        let iy1 = max(self.y1, other.y1);
        let ix2 = min(self.x2, other.x2);
        let iy2 = min(self.y2, other.y2);
        ix1 < ix2 && iy1 < iy2
    }
}

fn main() {
    // The specification does not define a sentinel; we read until EOF.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut all_outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        // Preconditions say positive; if not, behavior is undefined. We still handle n==0 by emitting nothing.
        let mut rects: Vec<Rect> = Vec::with_capacity(n);
        for _ in 0..n {
            let x1: i32 = it.next().unwrap().parse().unwrap();
            let y1: i32 = it.next().unwrap().parse().unwrap();
            let x2: i32 = it.next().unwrap().parse().unwrap();
            let y2: i32 = it.next().unwrap().parse().unwrap();
            rects.push(Rect { x1, y1, x2, y2 }.normalize());
        }

        // Build a dependency graph:
        // If A contains B (strict) or A overlaps B (positive area), then A must come before B.
        // This is a narrow, consistent interpretation of "overlapping and containment rules"
        // that yields a deterministic ordering via topological sort with tie-breaking.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indeg: Vec<usize> = vec![0; n];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let a = rects[i];
                let b = rects[j];
                if a.contains_strict(&b) || a.overlaps_area(&b) {
                    adj[i].push(j);
                }
            }
        }

        // Deduplicate edges to keep indegree correct.
        for i in 0..n {
            adj[i].sort_unstable();
            adj[i].dedup();
        }
        for i in 0..n {
            for &j in &adj[i] {
                indeg[j] += 1;
            }
        }

        // Kahn's algorithm with deterministic tie-breaking:
        // choose the smallest original index among currently available rectangles.
        let mut used = vec![false; n];
        let mut order: Vec<usize> = Vec::with_capacity(n);

        for _step in 0..n {
            let mut pick: Option<usize> = None;
            for i in 0..n {
                if !used[i] && indeg[i] == 0 {
                    pick = Some(i);
                    break;
                }
            }

            if let Some(u) = pick {
                used[u] = true;
                order.push(u);
                for &v in &adj[u] {
                    if indeg[v] > 0 {
                        indeg[v] -= 1;
                    }
                }
            } else {
                // Cycle or ambiguous constraints; specification doesn't define behavior.
                // Narrowest reasonable behavior: output remaining rectangles in increasing index order.
                for i in 0..n {
                    if !used[i] {
                        used[i] = true;
                        order.push(i);
                    }
                }
                break;
            }
        }

        // Output: each rectangle's order printed on a new line.
        // The specification doesn't define whether indices are 0- or 1-based; choose 1-based as typical.
        let mut out = String::new();
        for idx in order {
            out.push_str(&(idx + 1).to_string());
            out.push('\n');
        }
        all_outputs.push(out);
    }

    // Sets of outputs are separated by a newline.
    // Avoid an extra trailing blank line beyond what each set already ends with.
    let mut final_out = String::new();
    for (k, s) in all_outputs.iter().enumerate() {
        if k > 0 {
            final_out.push('\n');
        }
        final_out.push_str(s);
    }
    print!("{final_out}");
}

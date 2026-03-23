use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Seg {
    a: Pt,
    b: Pt,
}

fn on_segment_interior(p: Pt, s: Seg) -> bool {
    // Collinear and within bounding box, but strictly not equal to endpoints.
    if p == s.a || p == s.b {
        return false;
    }
    let ax = s.a.x as i64;
    let ay = s.a.y as i64;
    let bx = s.b.x as i64;
    let by = s.b.y as i64;
    let px = p.x as i64;
    let py = p.y as i64;

    let cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    if cross != 0 {
        return false;
    }

    let (minx, maxx) = if ax <= bx { (ax, bx) } else { (bx, ax) };
    let (miny, maxy) = if ay <= by { (ay, by) } else { (by, ay) };

    px >= minx && px <= maxx && py >= miny && py <= maxy
}

fn orient(a: Pt, b: Pt, c: Pt) -> i64 {
    // Signed area * 2: >0 left, <0 right, 0 collinear
    let ax = a.x as i64;
    let ay = a.y as i64;
    let bx = b.x as i64;
    let by = b.y as i64;
    let cx = c.x as i64;
    let cy = c.y as i64;
    (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        // Read segments
        let mut segs: Vec<Seg> = Vec::with_capacity(n);
        let mut all_endpoints: Vec<Pt> = Vec::with_capacity(2 * n);
        for _ in 0..n {
            let x1: i32 = it.next().unwrap().parse().unwrap();
            let y1: i32 = it.next().unwrap().parse().unwrap();
            let x2: i32 = it.next().unwrap().parse().unwrap();
            let y2: i32 = it.next().unwrap().parse().unwrap();
            let a = Pt { x: x1, y: y1 };
            let b = Pt { x: x2, y: y2 };
            segs.push(Seg { a, b });
            all_endpoints.push(a);
            all_endpoints.push(b);
        }

        // Endpoint multiplicities across the whole dataset
        let mut mult: HashMap<Pt, usize> = HashMap::new();
        for &p in &all_endpoints {
            *mult.entry(p).or_insert(0) += 1;
        }
        let is_shared = |p: Pt, mult: &HashMap<Pt, usize>| -> bool {
            mult.get(&p).copied().unwrap_or(0) > 1
        };

        // Build adjacency by shared endpoints (exact coordinate equality)
        let mut point_to_segs: HashMap<Pt, Vec<usize>> = HashMap::new();
        for (i, s) in segs.iter().enumerate() {
            point_to_segs.entry(s.a).or_default().push(i);
            point_to_segs.entry(s.b).or_default().push(i);
        }

        let mut visited = vec![false; n];
        let mut digit_counts = [0usize; 10];
        let mut accounted_segments = 0usize;

        for start in 0..n {
            if visited[start] {
                continue;
            }

            // BFS/DFS to collect component segments
            let mut q = VecDeque::new();
            q.push_back(start);
            visited[start] = true;

            let mut comp: Vec<usize> = Vec::new();
            while let Some(si) = q.pop_front() {
                comp.push(si);
                let s = segs[si];
                for &p in &[s.a, s.b] {
                    if let Some(neis) = point_to_segs.get(&p) {
                        for &nj in neis {
                            if !visited[nj] {
                                visited[nj] = true;
                                q.push_back(nj);
                            }
                        }
                    }
                }
            }

            let m = comp.len();
            if m < 2 {
                continue; // will be counted as digit 1 later
            }

            // p = number of endpoint occurrences in component that lie at shared points (with multiplicity)
            let mut pcount = 0usize;
            for &si in &comp {
                let s = segs[si];
                if is_shared(s.a, &mult) {
                    pcount += 1;
                }
                if is_shared(s.b, &mult) {
                    pcount += 1;
                }
            }

            // Template matching
            let mut matched_digit: Option<usize> = None;

            // A. Digit 4
            if m == 2 && pcount == 2 {
                matched_digit = Some(4);
            }

            // B. Digit 9
            if matched_digit.is_none() && m == 4 && pcount == 6 {
                matched_digit = Some(9);
            }

            // C. Digit 0 / 8
            if matched_digit.is_none() && m == 4 && pcount == 8 {
                let mut any_interior = false;
                'outer_c: for &pt in &all_endpoints {
                    for &si in &comp {
                        if on_segment_interior(pt, segs[si]) {
                            any_interior = true;
                            break 'outer_c;
                        }
                    }
                }
                matched_digit = Some(if any_interior { 8 } else { 0 });
            }

            // D. Digit 3 / 7
            if matched_digit.is_none() && m == 3 && pcount == 4 {
                let mut any_interior = false;
                'outer_d: for &pt in &all_endpoints {
                    for &si in &comp {
                        let s = segs[si];
                        if is_shared(s.a, &mult) && is_shared(s.b, &mult) {
                            if on_segment_interior(pt, s) {
                                any_interior = true;
                                break 'outer_d;
                            }
                        }
                    }
                }
                matched_digit = Some(if any_interior { 3 } else { 7 });
            }

            // E. Digit 2 / 5 / 6
            if matched_digit.is_none() && m == 5 && pcount == 8 {
                // Narrowest reasonable interpretation from the spec:
                // - Identify the two "junction" segments as those having exactly one non-shared endpoint.
                // - For each such segment, take its non-shared endpoint as a "branch point".
                // - Compare the sidedness of each branch point with respect to the other junction segment.
                // - Map the sign pattern to {2,5,6} deterministically.
                let mut junctions: Vec<(usize, Pt)> = Vec::new(); // (seg index, non-shared endpoint)
                for &si in &comp {
                    let s = segs[si];
                    let a_shared = is_shared(s.a, &mult);
                    let b_shared = is_shared(s.b, &mult);
                    if a_shared ^ b_shared {
                        let branch = if a_shared { s.b } else { s.a };
                        junctions.push((si, branch));
                    }
                }

                if junctions.len() == 2 {
                    let (s1i, b1) = junctions[0];
                    let (s2i, b2) = junctions[1];
                    let s1 = segs[s1i];
                    let s2 = segs[s2i];

                    let o1 = orient(s2.a, s2.b, b1);
                    let o2 = orient(s1.a, s1.b, b2);

                    // Deterministic mapping:
                    // - If both are strictly on the same side (both >0 or both <0): digit 2
                    // - If one is collinear (0) and the other nonzero: digit 5
                    // - Otherwise (opposite sides or both collinear): digit 6
                    let d = if (o1 > 0 && o2 > 0) || (o1 < 0 && o2 < 0) {
                        2
                    } else if (o1 == 0) ^ (o2 == 0) {
                        5
                    } else {
                        6
                    };
                    matched_digit = Some(d);
                } else {
                    // If the structure doesn't match the minimal junction identification, treat as unrecognized.
                    matched_digit = None;
                }
            }

            if let Some(d) = matched_digit {
                digit_counts[d] += 1;
                accounted_segments += m;
            }
        }

        // Catch-all digit 1: all remaining segments not otherwise accounted for.
        if accounted_segments <= n {
            digit_counts[1] += n - accounted_segments;
        } else {
            // Should not happen under the spec; keep output non-negative.
            digit_counts[1] += 0;
        }

        // Output counts for digits 0..9
        for d in 0..10 {
            if d > 0 {
                print!(" ");
            }
            print!("{}", digit_counts[d]);
        }
        println!();
    }
}

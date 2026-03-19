use std::io::{self, Read};

const GRID: usize = 2001; // indices 0..=2000
const OFFSET: i32 = 1000;

// Bounded region: coordinates in [-1000, 1000] inclusive.
fn in_bounds(x: i32, y: i32) -> bool {
    (-1000..=1000).contains(&x) && (-1000..=1000).contains(&y)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n < 0 {
            break;
        }
        let n = n as usize;

        // Two orientations stored on a coarse grid.
        // The specification only states "two orthogonal segment orientations" and that checks alternate.
        // We model them as two boolean grids.
        let mut seg0 = vec![false; GRID * GRID];
        let mut seg1 = vec![false; GRID * GRID];

        for _ in 0..n {
            let tok = it.next().unwrap();
            let gx: i32 = it.next().unwrap().parse().unwrap();
            let gy: i32 = it.next().unwrap().parse().unwrap();

            let ix = (gx + OFFSET) as usize;
            let iy = (gy + OFFSET) as usize;
            let idx = iy * GRID + ix;

            // Narrow interpretation: any token not matching the first recognized one maps to the other.
            // This preserves "one of two orientations" without inventing more.
            if tok == "0" || tok == "A" || tok == "H" || tok == "-" {
                seg0[idx] = true;
            } else {
                seg1[idx] = true;
            }
        }

        // Initial state: fixed initial position and direction.
        // The specification does not provide numeric initial values; choose the narrowest reasonable:
        // start at origin, moving diagonally (+1,+1).
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        // Direction encoded as (dx, dy), each in {-1, +1}.
        let mut dx: i32 = 1;
        let mut dy: i32 = 1;

        // Alternating reflection check: false => check seg0, true => check seg1.
        let mut alt: bool = false;

        // Step counter for "after the first two steps" termination conditions.
        let mut steps: i32 = 0;

        // Cycle detection: "a specific position-and-direction state is reached"
        // Narrow interpretation: detect return to the initial state (x,y,dx,dy,alt) after at least 2 steps.
        // On detection, replace position with a fixed escape position and terminate.
        let init_state = (x, y, dx, dy, alt);

        loop {
            // Advance by one fixed diagonal step.
            x = x.wrapping_add(dx);
            y = y.wrapping_add(dy);
            steps += 1;

            // Perform exactly one reflection check; orientation alternates each step.
            let mut reflected = false;

            // Checked location: narrow interpretation uses the current lattice cell (x,y) mapped to grid.
            // Preconditions guarantee indices are valid for all referenced checks.
            if in_bounds(x, y) {
                let ix = (x + OFFSET) as usize;
                let iy = (y + OFFSET) as usize;
                let idx = iy * GRID + ix;

                if !alt {
                    if seg0[idx] {
                        // Fixed mapping dependent on orientation:
                        // For seg0, reflect across one axis: invert dx.
                        dx = -dx;
                        reflected = true;
                    }
                } else {
                    if seg1[idx] {
                        // For seg1, reflect across the other axis: invert dy.
                        dy = -dy;
                        reflected = true;
                    }
                }
            }

            // Toggle which orientation is checked next step.
            alt = !alt;

            // Termination conditions apply after the first two steps.
            if steps >= 2 {
                // Boundary termination: at boundary and most recent step did not reflect.
                if !reflected && (x == -1000 || x == 1000 || y == -1000 || y == 1000) {
                    break;
                }

                // Cycle detection: reaching the specific repeated state.
                if (x, y, dx, dy, alt) == init_state {
                    // Fixed escape position: narrow choice is (0,0) as a canonical escape.
                    x = 0;
                    y = 0;
                    break;
                }
            }

            // If the point exits the bounded region, the specification says termination occurs
            // "when the point exits the bounded region (under specific conditions)".
            // Narrow interpretation: do not terminate merely for being out-of-bounds unless boundary rule triggers.
            // However, to avoid infinite wandering outside, clamp termination when fully outside by 1 step.
            if !in_bounds(x, y) {
                break;
            }
        }

        out.push_str(&format!("{} {}\n", x, y));
    }

    print!("{out}");
}

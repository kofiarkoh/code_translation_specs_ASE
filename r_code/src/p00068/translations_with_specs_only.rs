// convex_hull_interior_count.rs
//
// Translates the given specification into a Rust command-line program.
// The program repeatedly reads datasets from standard input. Each dataset
// begins with an integer N. If N == 0 the program terminates (exit code 0).
// Otherwise N coordinate pairs follow, each in the scanf-style format "%lf,%lf"
// (for example: "1.23,4.56") separated by whitespace. For each dataset the
// program computes monotone-chain style lower and upper hull sequences that
// preserve collinear boundary points, and prints a single integer per dataset:
//   output = N - (th_down_n + th_up_n - 2)
// where th_down_n and th_up_n are the counts of points in the computed lower
// and upper hull sequences respectively.
//
// The implementation follows the specification assumptions:
// - Input is assumed well-formed (no robust error-recovery).
// - Intended N range is 0..=100. The code does not enforce an upper bound;
//   the specification assumes inputs respect that constraint.
//
// The hull construction preserves collinear points by removing a point from the
// working hull only when the turn is strictly clockwise (cross < 0). This
// matches the described "preserve collinear boundary points" behavior.

use std::io::{self, Read};
use std::process::exit;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn cross(a: Point, b: Point, c: Point) -> f64 {
    // Cross product (b - a) x (c - a)
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Per specification, input is assumed to be available and well-formed.
        // If reading fails, terminate with non-zero code.
        exit(1);
    }

    // Split on ASCII whitespace to mimic scanf behavior for successive tokens.
    // Each coordinate pair is expected in the form "<double>,<double>" as a single token.
    let mut tokens = input.split_whitespace();

    // Helper to get next token or exit (mimics lack of robust checking in the C spec).
    let mut next_token = || -> String {
        tokens
            .next()
            .unwrap_or_else(|| {
                // If input ends unexpectedly, exit with non-zero code.
                // The spec assumes well-formed inputs; failing here is acceptable.
                eprintln!("Unexpected end of input");
                exit(1);
            })
            .to_string()
    };

    loop {
        // Read N
        let token = next_token();
        // Parse integer N similar to scanf("%d", &N).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => {
                // Malformed integer input: spec says scanf failures are not checked.
                // We choose to terminate with an error.
                eprintln!("Failed to parse integer N from token '{}'", token);
                exit(1);
            }
        };

        if n == 0 {
            // Terminate normally
            exit(0);
        }

        if n < 0 {
            // The specification does not define negative N; treat as error.
            eprintln!("Negative N not supported: {}", n);
            exit(1);
        }

        // Read N coordinate tokens of form "<double>,<double>"
        let mut points: Vec<Point> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let tok = next_token();
            // Split on the first comma
            let mut parts = tok.splitn(2, ',');
            let xs = parts.next().unwrap(); // safe: splitn always yields at least one
            let ys = match parts.next() {
                Some(s) => s,
                None => {
                    eprintln!("Coordinate token missing comma: '{}'", tok);
                    exit(1);
                }
            };
            let x: f64 = match xs.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Failed to parse x from '{}'", xs);
                    exit(1);
                }
            };
            let y: f64 = match ys.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Failed to parse y from '{}'", ys);
                    exit(1);
                }
            };
            points.push(Point { x, y });
        }

        // Sort by x then y (non-decreasing)
        points.sort_by(|a, b| {
            let cx = a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal);
            if cx == std::cmp::Ordering::Equal {
                a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                cx
            }
        });

        // Build lower hull
        let mut lower: Vec<Point> = Vec::with_capacity(points.len());
        for &p in &points {
            while lower.len() >= 2 {
                let m = lower.len();
                let a = lower[m - 2];
                let b = lower[m - 1];
                // Preserve collinear points: remove only on strictly clockwise turn (cross < 0)
                if cross(a, b, p) < 0.0 {
                    lower.pop();
                } else {
                    break;
                }
            }
            lower.push(p);
        }

        // Build upper hull (iterate reversed)
        let mut upper: Vec<Point> = Vec::with_capacity(points.len());
        for &p in points.iter().rev() {
            while upper.len() >= 2 {
                let m = upper.len();
                let a = upper[m - 2];
                let b = upper[m - 1];
                // Preserve collinear points: remove only on strictly clockwise turn (cross < 0)
                if cross(a, b, p) < 0.0 {
                    upper.pop();
                } else {
                    break;
                }
            }
            upper.push(p);
        }

        let th_down_n = lower.len() as i32;
        let th_up_n = upper.len() as i32;

        // Compute output = N - (th_down_n + th_up_n - 2)
        let output = n - (th_down_n + th_up_n - 2);

        println!("{}", output);
        // Continue to next dataset
    }
}

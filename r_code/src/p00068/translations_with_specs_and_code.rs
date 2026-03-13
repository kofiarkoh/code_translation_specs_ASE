use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

/// Cross product (b - a) x (c - b)
/// Equivalent to: (b.x - a.x)*(c.y - b.y) - (b.y - a.y)*(c.x - b.x)
fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x)
}

fn parse_input_tokens(s: &str) -> Vec<String> {
    // Split on any whitespace to emulate scanf-like tokenization,
    // but we'll handle coordinate tokens that may span one or two whitespace tokens
    s.split_whitespace().map(|t| t.to_string()).collect()
}

/// Parse one coordinate pair from tokens iterator following scanf("%lf,%lf") semantics tolerant of:
/// - "x,y"
/// - "x, y" (space after comma -> tokens ["x,", "y"])
/// - "x ,y" (space before comma is uncommon for scanf format, but we'll accept if token ",y" occurs)
///
/// Returns Some((x,y)) and consumes the necessary tokens from the iterator, or None if parsing fails.
fn parse_pair_from_tokens(tokens: &mut std::slice::Iter<String>) -> Option<Point> {
    let first = tokens.next()?;
    if let Some((xs, ys)) = first.split_once(',') {
        // token was "x,y" (maybe with extra whitespace trimmed by split_whitespace)
        let x = xs.parse::<f64>().ok()?;
        let y = ys.parse::<f64>().ok()?;
        Some(Point { x, y })
    } else if first.ends_with(',') {
        // token like "x," and next token is "y"
        let xs = first.trim_end_matches(',');
        let x = xs.parse::<f64>().ok()?;
        let second = tokens.next()?;
        let y = second.parse::<f64>().ok()?;
        Some(Point { x, y })
    } else if first.starts_with(',') {
        // token like ",y" and previous whitespace split separated the comma; unlikely but handle:
        let ys = first.trim_start_matches(',');
        let second = tokens.next()?; // previous token would have been x, but we don't have it -> fail
        // This branch is unsafe to recover; return None
        let _ = second; // silence unused
        None
    } else {
        // Possibly the input was "x" and next token is ",y" or next token is ",", then y.
        // Handle case next token starts with ',' like next = ",y"
        let backup = tokens.next();
        match backup {
            Some(next_tok) => {
                if next_tok.starts_with(',') {
                    // next_tok is like ",y" or ",y"
                    let x = first.parse::<f64>().ok()?;
                    let ys = next_tok.trim_start_matches(',');
                    let y = ys.parse::<f64>().ok()?;
                    Some(Point { x, y })
                } else if next_tok == "," {
                    // tokens: "x" "," "y"
                    let x = first.parse::<f64>().ok()?;
                    let y_tok = tokens.next()?;
                    let y = y_tok.parse::<f64>().ok()?;
                    Some(Point { x, y })
                } else {
                    // Unexpected format
                    None
                }
            }
            None => None,
        }
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, behave like the C program would be undefined; here we just exit.
        return;
    }

    let tokens_vec = parse_input_tokens(&input);
    let mut tokens = tokens_vec.iter();

    loop {
        // Read N (zahyou_num)
        let n_token = match tokens.next() {
            Some(t) => t,
            None => break, // EOF or malformed -> stop
        };
        let zahyou_num = match n_token.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break, // malformed -> stop (similar to unchecked scanf)
        };
        if zahyou_num == 0 {
            break; // termination
        }
        if zahyou_num < 0 {
            // Follow spec assumptions: N in 0..100. If negative, treat as malformed and stop.
            break;
        }
        let n = zahyou_num as usize;

        // Read n coordinate pairs
        let mut points: Vec<Point> = Vec::with_capacity(n);
        for _ in 0..n {
            match parse_pair_from_tokens(&mut tokens) {
                Some(p) => points.push(p),
                None => {
                    // Malformed input -> stop processing further (mimic undefined behavior of C by terminating)
                    return;
                }
            }
        }

        // Sort by x then y (non-decreasing)
        points.sort_by(|a, b| {
            if a.x < b.x {
                std::cmp::Ordering::Less
            } else if a.x > b.x {
                std::cmp::Ordering::Greater
            } else {
                if a.y < b.y {
                    std::cmp::Ordering::Less
                } else if a.y > b.y {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });

        // Build lower hull (monotone chain), preserving collinear points on edges.
        // We pop while cross < 0 (i.e., strictly clockwise turn). This preserves collinear (cross == 0).
        let mut lower: Vec<Point> = Vec::new();
        for &p in &points {
            while lower.len() >= 2 {
                let a = lower[lower.len() - 2];
                let b = lower[lower.len() - 1];
                if cross(a, b, p) < 0.0 {
                    lower.pop();
                } else {
                    break;
                }
            }
            lower.push(p);
        }

        // Build upper hull from right to left, same pop condition to preserve collinear points.
        let mut upper: Vec<Point> = Vec::new();
        for &p in points.iter().rev() {
            while upper.len() >= 2 {
                let a = upper[upper.len() - 2];
                let b = upper[upper.len() - 1];
                if cross(a, b, p) < 0.0 {
                    upper.pop();
                } else {
                    break;
                }
            }
            upper.push(p);
        }

        // Compute and print output: N - (th_down_n + th_up_n - 2)
        // Note: This matches the specification. For N==1 this yields 1 as in the original C behavior.
        let th_down_n = lower.len() as i32;
        let th_up_n = upper.len() as i32;
        let output = (n as i32) - (th_down_n + th_up_n - 2);
        println!("{}", output);
    }
}

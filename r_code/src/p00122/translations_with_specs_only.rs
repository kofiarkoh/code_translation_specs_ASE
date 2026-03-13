// survivability.rs
//
// Transliteration of the given Specification into Rust.
// Notes:
// - The Specification mentions "12 predefined move offsets" but does not list them.
//   To remain faithful (fixed finite set of 12 offsets) we choose the narrowest,
//   simple, symmetric interpretation: all integer offsets (dx,dy) with Manhattan
//   distance |dx|+|dy| <= 2 except the zero offset (0,0). This yields exactly 12
//   distinct offsets: (±1,0),(0,±1),(±2,0),(0,±2),(±1,±1).
//   This choice preserves determinism and matches the stated count of 12 moves.
// - Grid coordinates are treated as i32 and must be in [0,9] per Spec. The program
//   does not attempt to validate or correct out-of-range input (undefined behavior
//   per Specification); it only enforces bounds when computing reachable cells.
// - Input is read as whitespace-separated integers until the sentinel start (0 0).
// - For each test case we print exactly one line: "OK" or "NA".
//
// The program uses only safe Rust.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, per spec behavior is undefined; we simply exit.
        return;
    }
    let mut iter = input.split_whitespace().map(|w| w.parse::<i32>());

    // Precompute the 12 move offsets: Manhattan distance <= 2, excluding (0,0).
    let mut moves: Vec<(i32, i32)> = Vec::with_capacity(12);
    for dx in -2..=2 {
        for dy in -2..=2 {
            let manhattan = dx.abs() + dy.abs();
            if manhattan >= 1 && manhattan <= 2 {
                moves.push((dx, dy));
            }
        }
    }
    // Sanity: should be exactly 12 by this construction.
    debug_assert_eq!(moves.len(), 12);

    // Helper to fetch next parsed integer; if parsing fails or input ends, return None.
    let mut next_int = || -> Option<i32> {
        loop {
            match iter.next() {
                Some(Ok(v)) => return Some(v),
                Some(Err(_)) => return None,
                None => return None,
            }
        }
    };

    // Process test cases until sentinel (0,0) start coordinate is encountered.
    loop {
        let sx = match next_int() {
            Some(v) => v,
            None => break, // EOF or parse error -> stop processing
        };
        let sy = match next_int() {
            Some(v) => v,
            None => break,
        };

        // Sentinel: terminate processing and do not process this pair.
        if sx == 0 && sy == 0 {
            break;
        }

        // Read n
        let n = match next_int() {
            Some(v) => v,
            None => break,
        };
        let n_usize = if n >= 0 {
            // Spec assumes 0 <= n <= 10. If negative, behavior is undefined; we attempt to handle by stopping.
            n as usize
        } else {
            // Undefined per spec; stop processing.
            break;
        };

        // Read n scout coordinate pairs.
        let mut scouts: Vec<(i32, i32)> = Vec::with_capacity(n_usize);
        let mut valid_reads = true;
        for _ in 0..n_usize {
            let scx = match next_int() {
                Some(v) => v,
                None => { valid_reads = false; break; }
            };
            let scy = match next_int() {
                Some(v) => v,
                None => { valid_reads = false; break; }
            };
            scouts.push((scx, scy));
        }
        if !valid_reads {
            break;
        }

        // Per-spec initial candidate set is the single start coordinate.
        // We do not validate that start is within [0,9] (spec says undefined if not).
        let mut prev: Vec<(i32, i32)> = Vec::with_capacity(9);
        prev.push((sx, sy));

        let mut survived = true;

        for &(scx, scy) in &scouts {
            // next candidate set: cells in 3x3 neighborhood of (scx,scy)
            // that are reachable from any previous candidate by exactly one move offset,
            // and lie within grid bounds [0,9].
            let mut seen = [[false; 10]; 10];
            let mut next: Vec<(i32, i32)> = Vec::with_capacity(9);

            for &(px, py) in &prev {
                for &(dx, dy) in &moves {
                    let nx = px + dx;
                    let ny = py + dy;
                    // enforce grid bounds [0,9]
                    if nx < 0 || nx > 9 || ny < 0 || ny > 9 {
                        continue;
                    }
                    // must be within 3x3 neighborhood centered at scout coordinate
                    if (nx - scx).abs() <= 1 && (ny - scy).abs() <= 1 {
                        let xi = nx as usize;
                        let yi = ny as usize;
                        if !seen[xi][yi] {
                            seen[xi][yi] = true;
                            next.push((nx, ny));
                        }
                    }
                }
            }

            if next.is_empty() {
                // immediate failure per spec
                println!("NA");
                survived = false;
                break;
            }

            prev = next;
        }

        if survived {
            // After processing all scouts, if candidate set non-empty -> OK
            println!("OK");
        }
    }
}

use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i64 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // Undefined behavior per spec; choose to stop.
        };

        // Termination condition is not specified; the narrowest reasonable choice:
        // stop processing when n <= 0 (also aligns with "will not produce valid output").
        if n <= 0 {
            break;
        }

        let n_usize = n as usize;

        let mut xs: Vec<i64> = Vec::with_capacity(n_usize);
        let mut ys: Vec<i64> = Vec::with_capacity(n_usize);

        for _ in 0..n_usize {
            let x: i64 = it.next().unwrap().parse().unwrap();
            let y: i64 = it.next().unwrap().parse().unwrap();
            xs.push(x);
            ys.push(y);
        }

        // Read 4 corner points of the window.
        // Maintain min/max bounds as per invariant.
        let mut min_x: i64 = 0;
        let mut max_x: i64 = 0;
        let mut min_y: i64 = 0;
        let mut max_y: i64 = 0;

        for i in 0..4 {
            let x: i64 = it.next().unwrap().parse().unwrap();
            let y: i64 = it.next().unwrap().parse().unwrap();
            if i == 0 {
                min_x = x;
                max_x = x;
                min_y = y;
                max_y = y;
            } else {
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }

        // Polygon area via shoelace formula (closed loop).
        // Use i128 for intermediate products to avoid overflow; final output is long long (i64).
        let mut twice_poly_area: i128 = 0;
        for i in 0..n_usize {
            let j = (i + 1) % n_usize;
            twice_poly_area += (xs[i] as i128) * (ys[j] as i128) - (xs[j] as i128) * (ys[i] as i128);
        }
        let poly_area_abs: i128 = twice_poly_area.abs() / 2;

        // Window area from bounds.
        let w: i128 = (max_x as i128) - (min_x as i128);
        let h: i128 = (max_y as i128) - (min_y as i128);
        let win_area: i128 = w.abs() * h.abs();

        // Area difference: polygon - window.
        let diff: i128 = poly_area_abs - win_area;

        // Print as "long long integer" (i64). If out of range, saturate to i64 bounds
        // rather than inventing wraparound semantics not specified.
        let out_i64: i64 = if diff > i64::MAX as i128 {
            i64::MAX
        } else if diff < i64::MIN as i128 {
            i64::MIN
        } else {
            diff as i64
        };

        outputs.push(out_i64.to_string());
    }

    print!("{}", outputs.join("\n"));
}

use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as signed integers (like scanf("%ld")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(s) => s.parse::<i64>().unwrap(),
        None => return,
    };

    // C uses a VLA: long a[n]; If n <= 0, behavior is undefined in C.
    // We emulate a reasonable behavior: allocate max(n, 0) elements.
    let len = if n > 0 { n as usize } else { 0usize };
    let mut a: Vec<i64> = vec![0; len];

    for i in 0..len {
        let v = it.next().unwrap().parse::<i64>().unwrap();
        // a[i] -= i+1; with C signed overflow being UB; use wrapping to mimic typical 2's complement.
        a[i] = v.wrapping_sub((i as i64).wrapping_add(1));
    }

    // qsort with cmpnum: ascending numeric order.
    a.sort_unstable_by(|x, y| {
        if x > y {
            Ordering::Greater
        } else if x < y {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    // The C code assumes n >= 1; if n == 0, it would read a[0] (UB).
    // We'll avoid panics and print 0 in that case.
    if len == 0 {
        print!("0\n");
        return;
    }

    let mut all: i64 = 0;
    for i in 1..len {
        all = all.wrapping_add(a[i].wrapping_sub(a[0]));
    }

    let mut sadness: i64 = all;
    let mut minv: i64 = all;
    let mut level: i64 = a[0];
    let mut width: i64;
    let mut height: i64;

    // The C loop reads a[i+1] even when i == n-1 (out of bounds, UB).
    // To preserve intended behavior, we only iterate while i+1 < n.
    for i in 0..(len - 1) {
        if a[i + 1] == level {
            continue;
        }
        width = (i as i64).wrapping_add(1);

        // if (n-width*2 <= 0) break;
        let n_minus_2w = n.wrapping_sub(width.wrapping_mul(2));
        if n_minus_2w <= 0 {
            break;
        }

        height = a[i + 1].wrapping_sub(a[i]);

        // if (height*(n-width*2) <= sadness)
        let prod = height.wrapping_mul(n_minus_2w);
        if prod <= sadness {
            level = a[i + 1];
            sadness = sadness.wrapping_sub(prod);
            minv = sadness;
        } else {
            // height = sadness/(n-width*2);
            // C integer division truncates toward zero; Rust matches that for i64.
            height = sadness / n_minus_2w;
            level = level.wrapping_add(height);
            sadness = sadness.wrapping_sub(height.wrapping_mul(n_minus_2w));

            if sadness <= minv {
                minv = sadness;
            }

            // sadness = -(sadness-(n-width*2));
            sadness = -sadness.wrapping_sub(-n_minus_2w);

            if sadness <= minv {
                minv = sadness;
            }
            break;
        }
    }

    print!("{}\n", minv);
}

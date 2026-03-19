use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior for malformed input is undefined per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // If missing, unwrap will panic; spec leaves invalid input undefined.
    let n_i64: i64 = it.next().unwrap().parse().unwrap();

    // Spec: if n is negative, behavior is not defined. We do not add handling.
    let n: usize = n_i64 as usize;

    // Read exactly n heights; if fewer/more/invalid, behavior is undefined.
    let mut heights: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let h: i64 = it.next().unwrap().parse().unwrap();
        heights.push(h);
    }

    // Largest rectangle in histogram in O(n) using a monotonic stack of indices.
    // We use i128 for area to avoid overflow assumptions; spec does not define overflow behavior.
    let mut stack: Vec<usize> = Vec::with_capacity(n);
    let mut max_area: i128 = 0;

    for i in 0..=n {
        let cur_h: i64 = if i == n { 0 } else { heights[i] };

        while let Some(&top) = stack.last() {
            if heights[top] <= cur_h {
                break;
            }
            let h = heights[top] as i128;
            stack.pop();

            let left_boundary_exclusive = stack.last().map(|&idx| idx).unwrap_or(usize::MAX);
            let width: i128 = if left_boundary_exclusive == usize::MAX {
                i as i128
            } else {
                (i - left_boundary_exclusive - 1) as i128
            };

            let area = h * width;
            if area > max_area {
                max_area = area;
            }
        }

        stack.push(i);
    }

    // Output as an integer. Spec does not constrain integer width; print as i128.
    println!("{}", max_area);
}

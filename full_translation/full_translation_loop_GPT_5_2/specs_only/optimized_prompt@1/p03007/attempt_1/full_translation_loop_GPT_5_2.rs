use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // No input; spec doesn't define behavior, so exit quietly.
    };

    // Spec: at least one integer.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(x) = it.next() {
            a.push(x.parse::<i64>().unwrap());
        } else {
            // Incomplete input; spec doesn't define behavior. Stop early.
            break;
        }
    }

    if a.is_empty() {
        return;
    }

    // Compute absolute sum using i64 (long long). Use wrapping to emulate typical C++ long long
    // overflow behavior as closely as possible (though C++ signed overflow is UB).
    let mut abs_sum: i64 = 0;
    for &v in &a {
        // i64::abs overflows on MIN; emulate two's-complement wrapping like many platforms.
        let av = if v == i64::MIN { v } else { v.abs() };
        abs_sum = abs_sum.wrapping_add(av);
    }

    // Determine sign categories.
    let mut any_pos = false;
    let mut any_neg = false;
    for &v in &a {
        if v > 0 {
            any_pos = true;
        } else if v < 0 {
            any_neg = true;
        }
    }

    // The specification describes three states and "a series of pairs ... in a specific order",
    // but does not define the exact computed value beyond "absolute sum" nor the exact pair
    // generation rules. The narrowest faithful interpretation is:
    // - Print the absolute sum.
    // - Print pairs that preserve input order and reflect a relationship between the running
    //   absolute sum and each element.
    //
    // We therefore output, for each element in order, a pair:
    //   (prefix_abs_sum_after_including_element, element_value)
    // This is consistent with:
    // - "absolute sum ... remains consistent with the values processed"
    // - "order ... corresponds to their order in the input list"
    // - "series of pairs ... based on the computations performed on the list"
    //
    // The three states are acknowledged but do not change output format because the spec does
    // not provide distinct pair rules per state.
    let _state = if !any_neg {
        0 // all non-negative
    } else if !any_pos {
        1 // all non-positive
    } else {
        2 // mix
    };

    let mut out = String::new();
    out.push_str(&format!("{}\n", abs_sum));

    let mut prefix_abs: i64 = 0;
    for &v in &a {
        let av = if v == i64::MIN { v } else { v.abs() };
        prefix_abs = prefix_abs.wrapping_add(av);
        out.push_str(&format!("{} {}\n", prefix_abs, v));
    }

    print!("{out}");
}

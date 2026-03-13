// triangle_contains.rs
//
// Translates the provided specification to Rust.
// Behavior notes:
// - Integer arithmetic models C `int` with two's-complement wraparound semantics.
//   To emulate that, all multiplications and subtractions used to compute cross
//   products use wrapping operations on i32.
// - Input is read as whitespace-separated integers. The program assumes the
//   input contains at least one integer `n` followed by exactly 10*n integers,
//   matching the specification. Parsing failures will cause a panic (similar to
//   undefined behavior in the original C program when scanf fails).
//
// Output: For each test case, prints exactly one line: "OK" or "NG".

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Parse all tokens as i32. Per specification, tokens are C int; we use i32.
    let mut toks = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Read n
    let n_i32 = match toks.next() {
        Some(v) => v,
        None => return, // no input; spec assumes n is present, but silently exit if not
    };

    // If n <= 0, no iterations (spec: negative n results in no iterations).
    if n_i32 <= 0 {
        return;
    }
    let n = n_i32 as usize;

    for _ in 0..n {
        // Read 10 integers for the test case. Using unwrap to mirror spec's lack of input validation.
        let x1 = toks.next().unwrap();
        let y1 = toks.next().unwrap();
        let x2 = toks.next().unwrap();
        let y2 = toks.next().unwrap();
        let x3 = toks.next().unwrap();
        let y3 = toks.next().unwrap();
        let xk = toks.next().unwrap();
        let yk = toks.next().unwrap();
        let xs = toks.next().unwrap();
        let ys = toks.next().unwrap();

        let k_inside = point_in_triangle_inclusive_wrapping(x1, y1, x2, y2, x3, y3, xk, yk);
        let s_inside = point_in_triangle_inclusive_wrapping(x1, y1, x2, y2, x3, y3, xs, ys);

        // Exactly one contained -> OK, else NG
        if k_inside ^ s_inside {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}

/// Computes the 2D cross product (B - A) x (P - A) using wrapping arithmetic on i32.
/// This emulates C `int` two's-complement overflow semantics as closely as possible.
#[inline]
fn cross_wrapping(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> i32 {
    let bx_ax = bx.wrapping_sub(ax);
    let by_ay = by.wrapping_sub(ay);
    let px_ax = px.wrapping_sub(ax);
    let py_ay = py.wrapping_sub(ay);

    // (bx - ax)*(py - ay) - (by - ay)*(px - ax)
    bx_ax.wrapping_mul(py_ay).wrapping_sub(by_ay.wrapping_mul(px_ax))
}

/// Returns true if point P is contained in the triangle ABC (including boundary),
/// using the "same side" checks against each edge. All arithmetic uses wrapping
/// i32 operations to emulate C `int` overflow behavior.
fn point_in_triangle_inclusive_wrapping(
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    cx: i32,
    cy: i32,
    px: i32,
    py: i32,
) -> bool {
    // For edge AB, check sign of cross(AB, P) vs cross(AB, C)
    let cp_p_ab = cross_wrapping(ax, ay, bx, by, px, py);
    let cp_c_ab = cross_wrapping(ax, ay, bx, by, cx, cy);
    if !same_sign_or_zero(cp_p_ab, cp_c_ab) {
        return false;
    }

    // For edge BC, check sign of cross(BC, P) vs cross(BC, A)
    let cp_p_bc = cross_wrapping(bx, by, cx, cy, px, py);
    let cp_a_bc = cross_wrapping(bx, by, cx, cy, ax, ay);
    if !same_sign_or_zero(cp_p_bc, cp_a_bc) {
        return false;
    }

    // For edge CA, check sign of cross(CA, P) vs cross(CA, B)
    let cp_p_ca = cross_wrapping(cx, cy, ax, ay, px, py);
    let cp_b_ca = cross_wrapping(cx, cy, ax, ay, bx, by);
    if !same_sign_or_zero(cp_p_ca, cp_b_ca) {
        return false;
    }

    true
}

/// Returns true if two i32 values have the same sign or at least one is zero.
/// This uses standard signed comparisons on i32; wrapped arithmetic was applied
/// before calling this for cross products.
#[inline]
fn same_sign_or_zero(a: i32, b: i32) -> bool {
    if a == 0 || b == 0 {
        true
    } else {
        (a > 0 && b > 0) || (a < 0 && b < 0)
    }
}

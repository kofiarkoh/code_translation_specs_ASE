// Translated from C (Aizu Vol0 0049: Blood Groups).
// Behavior preserved from the original C program:
// - Parses lines of the form "<k> , <TYPE>" where TYPE is A, B, AB, or O.
// - For each k (expected in 1..=50) records the first seen blood type and increments counts.
// - If a different blood type for the same k is encountered, the program loops forever (as the original did).
// Notes:
// - This translation tries to follow the C parsing logic precisely (including skipping whitespaces
//   in the same places). To safely index bytes like C's NUL-terminated strings, each input line
//   is stripped of trailing newline(s) and a NUL sentinel is appended so indexing past the end
//   behaves like reading a '\0' in C.
// - The implementation assumes input k values are within the intended range; out-of-range lines
//   are ignored rather than causing undefined memory access.

use std::io::{self, Read};

const MAX: usize = 50;

fn main() {
    // blood[0..MAX+4] to match C's int blood[MAX + 5];
    let mut blood = [0i32; MAX + 5];
    // cnt[0..4], using indices 1..4
    let mut cnt = [0i32; 5];

    // Read entire stdin into a string, then iterate over lines similar to gets loop.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, just print zeros (though original would behave differently).
        for k in 1..5 {
            println!("{}", cnt[k]);
        }
        return;
    }

    for raw_line in input.lines() {
        // Emulate gets: lines() removes the trailing newline, so raw_line corresponds to the buffer contents.
        // We append a NUL byte to emulate C-style string accesses past the end returning '\0'.
        let mut buf = Vec::with_capacity(raw_line.len() + 1);
        buf.extend_from_slice(raw_line.as_bytes());
        buf.push(b'\0');

        // parser pointer index
        let mut i = 0usize;
        // skip leading whitespace
        while buf[i].is_ascii_whitespace() {
            i += 1;
        }

        // k = *p++ - '0';  (C did this without checking isdigit for the first char)
        let mut k_val = (buf[i] as i32) - (b'0' as i32);
        i += 1;

        // if (isdigit(*p)) k = k * 10 + (*p++ - '0');
        if buf[i].is_ascii_digit() {
            k_val = k_val * 10 + ((buf[i] as i32) - (b'0' as i32));
            i += 1;
        }

        // while (isspace(*p)) p++;  // ','
        while buf[i].is_ascii_whitespace() {
            i += 1;
        }
        // p++;  // skip the comma (or whatever char is there)
        i = i.wrapping_add(1);

        // while (isspace(*p)) p++;
        while buf[i].is_ascii_whitespace() {
            i += 1;
        }

        // determine t:
        // if *p == 'A' && *(p+1) == 'B' => 3
        // else if *p == 'B' => 2
        // else if *p == 'O' => 4
        // else => 1 (A)
        let t = if buf[i] == b'A' && buf.get(i + 1).copied().unwrap_or(b'\0') == b'B' {
            3
        } else if buf[i] == b'B' {
            2
        } else if buf[i] == b'O' {
            4
        } else {
            1
        };

        // Mirror C's use of k as an int. If k_val is negative or out of array bounds, the original C
        // would have undefined behavior. Here we guard: if k is out of expected bounds, skip the line.
        // This keeps the program safe while matching intended usage for valid inputs.
        if k_val < 0 {
            continue;
        }
        let k_usize = k_val as usize;
        if k_usize >= blood.len() {
            continue;
        }

        if blood[k_usize] == 0 {
            blood[k_usize] = t;
            cnt[t as usize] += 1;
        } else if blood[k_usize] != t {
            // Preserve the original program's behavior on conflict: infinite loop.
            loop {}
        }
    }

    for k in 1..5 {
        println!("{}", cnt[k]);
    }
}

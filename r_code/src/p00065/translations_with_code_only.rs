use std::io::{self, Read};

fn is_whitespace_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\t' | b'\r' | 0x0b | 0x0c)
}

// Parse an integer starting at s[idx], following scanf "%d" semantics:
// - Skip leading whitespace before the integer is handled by caller when appropriate.
// - Accept optional '+' or '-' sign.
// - Require at least one digit.
// Returns (value_as_i32_wrapped, new_index) on success, or None if no integer found.
fn parse_int_wrapping(s: &[u8], mut idx: usize) -> Option<(i32, usize)> {
    let len = s.len();
    if idx >= len {
        return None;
    }

    // Optional sign
    let mut negative = false;
    if s[idx] == b'+' {
        idx += 1;
    } else if s[idx] == b'-' {
        negative = true;
        idx += 1;
    }

    // Must have at least one digit
    if idx >= len || !(b'0'..=b'9').contains(&s[idx]) {
        return None;
    }

    // Parse into i64 to avoid intermediate overflow, then wrap to i32 at the end.
    let mut val: i64 = 0;
    while idx < len {
        let c = s[idx];
        if !(b'0'..=b'9').contains(&c) {
            break;
        }
        val = val * 10 + (c - b'0') as i64;
        idx += 1;
    }

    if negative {
        val = -val;
    }

    // Wrap into i32 like C's typical 32-bit two's complement casting.
    let v32 = val as i32;
    Some((v32, idx))
}

fn main() {
    // Read entire stdin into a buffer
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let bytes = input.as_bytes();
    let mut idx: usize = 0;
    let len = bytes.len();

    // Emulate the C arrays of size 10000 and their behavior (but avoid UB by capping).
    const MAX_ENTRIES: usize = 10000;
    let mut number: Vec<i32> = Vec::with_capacity(MAX_ENTRIES);
    let mut _ac: Vec<i32> = Vec::with_capacity(MAX_ENTRIES); // second values stored but unused

    // The original C code used scanf("%d,%d") repeatedly until EOF.
    // We emulate scanf semantics:
    // - For the first %d: skip leading whitespace, parse integer.
    // - Next in format is a literal ',' (no skipping of intermediate whitespace).
    // - Then for the second %d: skip leading whitespace, parse integer.
    while idx < len && number.len() < MAX_ENTRIES {
        // Skip leading whitespace before the first %d (scanf behavior for %d)
        while idx < len && is_whitespace_byte(bytes[idx]) {
            idx += 1;
        }

        // Parse first integer
        let (first, new_idx) = match parse_int_wrapping(bytes, idx) {
            Some(v) => v,
            None => break, // scanf would fail to match -> stop at EOF/invalid like the C code
        };
        idx = new_idx;

        // Next character must be a literal comma (scanf "%d,%d" does not skip whitespace here).
        if idx >= len || bytes[idx] != b',' {
            break;
        }
        idx += 1; // consume comma

        // For the second %d, skip leading whitespace
        while idx < len && is_whitespace_byte(bytes[idx]) {
            idx += 1;
        }

        // Parse second integer
        let (second, new_idx2) = match parse_int_wrapping(bytes, idx) {
            Some(v) => v,
            None => break,
        };
        idx = new_idx2;

        number.push(first);
        _ac.push(second);
    }

    let count = number.len();

    // Replicate the C sorting logic:
    // for(i=0;i<count;i++){
    //   for(j=0;j<count;j++){
    //     if(number[i]<number[j]) swap
    //   }
    // }
    for i in 0..count {
        for j in 0..count {
            if number[i] < number[j] {
                number.swap(i, j);
            }
        }
    }

    // Replicate the duplicate-counting loop exactly, including index adjustments.
    // C:
    // for(i=0,k=0;i<count;i++){
    //   for(j=i;j<count;j++){
    //     if(number[i]==number[j] && i!=j)k++;
    //   }
    //   i+=k;
    //   if(k>=1)printf("%d %d\n",number[i],k+1);
    //   k=0;
    // }
    let mut i: usize = 0;
    while i < count {
        let mut k: usize = 0;
        let ni = i;
        let mut j = i;
        while j < count {
            if number[ni] == number[j] && ni != j {
                k += 1;
            }
            j += 1;
        }

        // Advance i by k (to index of last duplicate), then the outer loop would increment i by 1.
        // We emulate by doing i += k; then printing if needed; then i += 1 at end of loop.
        i = i + k;
        if k >= 1 {
            // Print exactly like printf("%d %d\n", number[i], k+1);
            println!("{} {}", number[i], k + 1);
        }
        // reset k (not necessary in this scope)
        i += 1;
    }
}

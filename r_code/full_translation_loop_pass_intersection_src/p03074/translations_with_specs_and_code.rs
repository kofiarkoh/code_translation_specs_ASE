use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace (matches scanf("%d %d") and scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let num: usize = it.next().unwrap().parse().unwrap();
    let changes: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap_or("");

    // Convert first `num` chars: '1' => true, otherwise false (per spec invariant).
    // The C code assumes s has at least `num` chars (precondition).
    let bytes = s.as_bytes();
    let mut bits: Vec<u8> = Vec::with_capacity(num + 1);
    for i in 0..num {
        bits.push(if bytes[i] == b'1' { 1 } else { 0 });
    }

    // Sentinel: str[num] = !(str[num-1]) to force flushing the last run.
    // (C uses char with boolean values 0/1; we do the same with u8.)
    let last = bits[num - 1];
    bits.push(if last == 1 { 0 } else { 1 });

    // Build run-length array starting with an implicit previous value of 1,
    // producing runs in the pattern: #1 #0 #1 #0 ...
    let mut arr: Vec<i32> = Vec::new();
    let mut prev: u8 = 1;
    let mut cnt: i32 = 0;
    for &b in &bits {
        if b == prev {
            cnt += 1;
        } else {
            arr.push(cnt);
            prev = b;
            cnt = 1;
        }
    }
    let aidx = arr.len();

    // Prefix sums `cum` with the same indexing scheme as the C code:
    // cum[i] = sum_{k < i} arr[k]
    let mut cum: Vec<i32> = vec![0; aidx + 1];
    for i in 0..aidx {
        cum[i + 1] = cum[i] + arr[i];
    }

    let ranges: usize = 2 * changes + 1;
    let mut ans: i32 = 0;

    // Consider windows starting at '1'-run boundaries (even indices).
    // Note: The C code computes cum[i+ranges]-cum[i] before checking bounds,
    // which is undefined if i+ranges > aidx. Per spec, behavior is undefined
    // when preconditions are violated; here we keep defined Rust behavior by
    // clamping the window end to aidx (equivalent to "use as many runs as exist").
    for i in (0..aidx).step_by(2) {
        let end = (i + ranges).min(aidx);
        let val = cum[end] - cum[i];
        if val > ans {
            ans = val;
        }
        if i + ranges > aidx {
            break;
        }
    }

    print!("{ans}\n");
}

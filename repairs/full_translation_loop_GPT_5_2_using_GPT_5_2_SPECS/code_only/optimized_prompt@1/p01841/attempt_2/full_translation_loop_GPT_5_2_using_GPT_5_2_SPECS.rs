use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct MyStr {
    sptr: usize, // start index into a backing byte buffer
    len: usize,
}

/// Parse like C's atoi: skip leading whitespace, optional sign, then digits.
/// If no digits, returns 0. Stops at first non-digit.
/// Overflow in C is undefined; we emulate common 2's-complement wrapping.
fn c_atoi(bytes: &[u8]) -> i32 {
    let mut i = 0;
    while i < bytes.len() && (bytes[i] as char).is_ascii_whitespace() {
        i += 1;
    }
    let mut sign: i32 = 1;
    if i < bytes.len() {
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        }
    }
    let mut any = false;
    let mut val: i32 = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if !(b'0'..=b'9').contains(&c) {
            break;
        }
        any = true;
        let d = (c - b'0') as i32;
        val = val.wrapping_mul(10).wrapping_add(d);
        i += 1;
    }
    if !any { 0 } else { val.wrapping_mul(sign) }
}

fn analyze_tree(input: &MyStr, left: &mut MyStr, right: &mut MyStr, val: &mut i32, buf: &[u8]) {
    let p = input.sptr;
    left.sptr = p;

    let mut lv: i32 = 0;
    for i in 0..input.len {
        let ch = buf[p + i];
        if ch == b'[' {
            if lv == 0 {
                left.len = i;
                *val = c_atoi(&buf[p + i + 1..p + input.len]);
            }
            lv += 1;
        } else if ch == b']' {
            if lv == 1 {
                right.sptr = p + i + 1;
                right.len = input.len - i - 1;
            }
            lv -= 1;
        } else if ch == b'(' {
            lv += 1;
        } else if ch == b')' {
            lv -= 1;
        }
    }
}

fn is_empty_tree(input: &MyStr) -> bool {
    input.len == 2
}

fn remove_paren(input: &mut MyStr) {
    input.len = input.len.saturating_sub(2);
    input.sptr += 1;
}

fn add_tree(in1: &MyStr, in2: &MyStr, out: &mut MyStr, buf: &[u8], ans: &mut Vec<u8>) {
    // Avoid borrow-checker issues by using separate locals instead of a 2D array.
    let mut l0 = MyStr::default();
    let mut r0 = MyStr::default();
    let mut l1 = MyStr::default();
    let mut r1 = MyStr::default();
    let mut v0: i32 = 0;
    let mut v1: i32 = 0;

    analyze_tree(in1, &mut l0, &mut r0, &mut v0, buf);
    analyze_tree(in2, &mut l1, &mut r1, &mut v1, buf);
    let val = v0.wrapping_add(v1);

    let mut sstree0 = MyStr::default();
    let mut sstree1 = MyStr::default();

    // Left subtree
    if is_empty_tree(&l0) || is_empty_tree(&l1) {
        sstree0 = MyStr { sptr: 0, len: 0 }; // corresponds to zeros/len=0 in C
    } else {
        let mut a = l0;
        let mut b = l1;
        remove_paren(&mut a);
        remove_paren(&mut b);
        add_tree(&a, &b, &mut sstree0, buf, ans);
    }

    // Right subtree
    if is_empty_tree(&r0) || is_empty_tree(&r1) {
        sstree1 = MyStr { sptr: 0, len: 0 };
    } else {
        let mut a = r0;
        let mut b = r1;
        remove_paren(&mut a);
        remove_paren(&mut b);
        add_tree(&a, &b, &mut sstree1, buf, ans);
    }

    // out->sptr = ap; in Rust, out points into `ans` buffer.
    out.sptr = ans.len();

    // Emulate: sprintf(ap,"(%.*s)[%d](%.*s)", sstree0, val, sstree1)
    ans.push(b'(');
    if sstree0.len > 0 {
        // Can't extend from `ans` while also borrowing it immutably; copy to temp.
        let tmp: Vec<u8> = ans[sstree0.sptr..sstree0.sptr + sstree0.len].to_vec();
        ans.extend_from_slice(&tmp);
    }
    ans.push(b')');
    ans.push(b'[');
    ans.extend_from_slice(val.to_string().as_bytes());
    ans.push(b']');
    ans.push(b'(');
    if sstree1.len > 0 {
        let tmp: Vec<u8> = ans[sstree1.sptr..sstree1.sptr + sstree1.len].to_vec();
        ans.extend_from_slice(&tmp);
    }
    ans.push(b')');

    out.len = ans.len() - out.sptr;
}

fn main() {
    // Read all input, then mimic scanf("%s") twice (whitespace-delimited tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t0 = it.next().unwrap_or("");
    let t1 = it.next().unwrap_or("");

    // Backing buffer for the two input trees (like input_tree[2][100000]).
    // We store them concatenated with a separator; indices are tracked explicitly.
    let mut buf: Vec<u8> = Vec::with_capacity(t0.len() + 1 + t1.len());
    let t0_start = 0usize;
    buf.extend_from_slice(t0.as_bytes());
    let t0_len = t0.len();
    buf.push(0); // separator (not used by logic, but keeps regions distinct)
    let t1_start = buf.len();
    buf.extend_from_slice(t1.as_bytes());
    let t1_len = t1.len();

    let tree0 = MyStr { sptr: t0_start, len: t0_len };
    let tree1 = MyStr { sptr: t1_start, len: t1_len };

    let mut ans: Vec<u8> = Vec::with_capacity(100_000);
    let mut tree2 = MyStr::default();

    add_tree(&tree0, &tree1, &mut tree2, &buf, &mut ans);

    // display: print exactly the computed bytes then '\n'
    let out_bytes = &ans[tree2.sptr..tree2.sptr + tree2.len];
    let mut stdout = Vec::with_capacity(out_bytes.len() + 1);
    stdout.extend_from_slice(out_bytes);
    stdout.push(b'\n');
    print!("{}", String::from_utf8_lossy(&stdout));
}

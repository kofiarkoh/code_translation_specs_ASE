use std::io::{self, Read};

const NUM_MAX: usize = 100000;
const BUF_SIZE: usize = NUM_MAX + 50;

fn main() {
    // Read entire stdin, then parse tokens like scanf("%d %d") and scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let num: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
    let changes: i32 = it.next().unwrap().parse::<i32>().unwrap();

    // get_str with "%<size>s" reads a whitespace-delimited token, up to size chars.
    // In the C code, size is BUF_SIZE, so truncation won't matter for valid inputs.
    let token = it.next().unwrap_or("");
    let mut str_buf = vec![0u8; BUF_SIZE];
    let bytes = token.as_bytes();
    let copy_len = bytes.len().min(BUF_SIZE);
    str_buf[..copy_len].copy_from_slice(&bytes[..copy_len]);

    // for(i = 0; i < num; i++) str[i] = (str[i] == '1');
    // str[num] = !(str[num-1]);
    // Use u8 0/1 to mimic C char used as boolean.
    for i in 0..num {
        str_buf[i] = if str_buf[i] == b'1' { 1 } else { 0 };
    }
    // C assumes num >= 1; we preserve that assumption (would be UB in C otherwise).
    str_buf[num] = if str_buf[num - 1] == 0 { 1 } else { 0 };

    // Build run-length array arr.
    // static int arr[NUM_MAX*4];
    let mut arr = vec![0i32; NUM_MAX * 4];
    let mut aidx: usize = 0;
    let mut prev: u8 = 1;
    let mut cnt: i32 = 0;

    for i in 0..=num {
        if str_buf[i] == prev {
            cnt += 1;
            continue;
        }
        arr[aidx] = cnt;
        aidx += 1;
        prev = str_buf[i];
        cnt = 1;
    }

    // Build cumulative sums cum where cum[i] = sum_{k < i} arr[k]
    // static int cum[NUM_MAX*4];
    let mut cum = vec![0i32; NUM_MAX * 4];
    for i in 1..(NUM_MAX * 4) {
        cum[i] = cum[i - 1] + arr[i - 1];
    }

    let mut ans: i32 = 0;
    let ranges: usize = (2i32 * changes + 1) as usize;

    // for(i = 0; i < aidx; i+=2) {
    //     ans = max(ans, cum[i+ranges] - cum[i]);
    //     if(i+ranges > aidx) break;
    // }
    // Note: The C code computes cum[i+ranges] even when i+ranges > aidx,
    // but cum is sized to NUM_MAX*4 and arr beyond aidx is zero-initialized,
    // so this is well-defined as long as i+ranges < NUM_MAX*4.
    // We preserve that behavior.
    let mut i: usize = 0;
    while i < aidx {
        let idx = i + ranges;
        if idx < cum.len() {
            let val = cum[idx] - cum[i];
            if val > ans {
                ans = val;
            }
        }
        if i + ranges > aidx {
            break;
        }
        i += 2;
    }

    print!("{ans}\n");
}

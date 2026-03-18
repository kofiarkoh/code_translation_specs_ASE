use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic scanf("%s", s): take first whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = match input.split_whitespace().next() {
        Some(tok) => tok,
        None => return, // If no token, C's scanf would fail; we just exit.
    };

    // C: int i,l,a=0,c=0,b[256]={0}; char s[100];
    // We emulate b[256] indexed by unsigned byte values.
    let mut b = [0i32; 256];
    let bytes = s.as_bytes();

    // C loop: for(l=0;s[l];l++) b[s[l]]++;
    // In C, char may be signed/unsigned; indexing b[s[l]] is UB if char is negative.
    // Here we use bytes (0..=255) which matches the intended "byte frequency" behavior.
    let mut l: i32 = 0;
    for &ch in bytes {
        b[ch as usize] += 1;
        l += 1;
    }

    let mut a: i32 = 0;
    let mut c: i32 = 0;

    // C: while(l){ for(i=0;i<256;i++){ ... } }
    while l != 0 {
        for i in 0..256usize {
            if b[i] != 0 {
                a += 1;
                l -= 1;
                c += 1;
                b[i] -= 1;
            } else {
                if c > 3 {
                    a -= c - 3;
                }
                c = 0;
            }
        }
    }

    // C: printf("%d\n",a);
    print!("{a}\n");
}

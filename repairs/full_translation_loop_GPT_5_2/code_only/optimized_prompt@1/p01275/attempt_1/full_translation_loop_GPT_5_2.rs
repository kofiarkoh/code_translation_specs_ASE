use std::io::{self, Read};

static mut N: usize = 0;
static mut MIN_: i32 = 0;

fn find_not_zero(diff: &[u8], start: usize) -> i32 {
    // C: for(i=start;i<n;i++) if(diff[i]) return(i); return(-1);
    let n = unsafe { N };
    for i in start..n {
        if diff[i] != 0 {
            return i as i32;
        }
    }
    -1
}

fn rotate_count(diff: &[u8; 10], acc: i32) {
    // Uses globals N and MIN_ like the C code.
    let n = unsafe { N };

    let mut pos: i32 = find_not_zero(diff, 0);
    if pos == -1 {
        unsafe {
            if MIN_ > acc {
                MIN_ = acc;
            }
        }
        return;
    }

    let pos_usize = pos as usize;
    let offs = diff[pos_usize];

    // C: memcpy(diff2,diff,10);
    let mut diff2 = *diff;

    for i in pos_usize..n {
        // C:
        // diff2[i]=diff2[i]+10-offs;
        // diff2[i] %= 10;
        // All values are 0..9, so u8 arithmetic is safe here.
        let v = diff2[i] + 10 - offs;
        diff2[i] = v % 10;
        rotate_count(&diff2, acc + 1);
    }
}

fn calc_diff(init: &[u8], unlock: &[u8], diff: &mut [u8; 10], n: usize) {
    // C computes digit differences modulo 10.
    for i in 0..n {
        let mut d = unlock[i] as i16 - init[i] as i16;
        if d < 0 {
            d += 10;
        }
        diff[i] = d as u8;
    }
}

fn main() {
    // Read entire stdin and tokenize like scanf("%d") / scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n_val: i32 = n_opt.unwrap().parse().unwrap();
        if n_val == 0 {
            break;
        }
        let n = n_val as usize;
        unsafe { N = n; }

        let init_s = it.next().unwrap_or("");
        let unlock_s = it.next().unwrap_or("");

        // C uses char arrays of size 10 and reads with %s (no bounds check).
        // AOJ constraints keep it within 10; we mirror by truncating/padding to 10.
        let mut init = [0u8; 10];
        let mut unlock = [0u8; 10];
        for (i, b) in init_s.as_bytes().iter().take(10).enumerate() {
            init[i] = *b;
        }
        for (i, b) in unlock_s.as_bytes().iter().take(10).enumerate() {
            unlock[i] = *b;
        }

        // C: if(0==memcmp(init,unlock,n)) printf("0\n");
        if init[..n] == unlock[..n] {
            out.push_str("0\n");
        } else {
            unsafe { MIN_ = 99; }
            let mut diff = [0u8; 10];
            calc_diff(&init, &unlock, &mut diff, n);
            rotate_count(&diff, 0);
            let minv = unsafe { MIN_ };
            out.push_str(&format!("{minv}\n"));
        }
    }

    print!("{out}");
}

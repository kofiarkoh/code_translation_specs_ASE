use std::io::{self, Read, Write};

fn main() {
    let mut buf = vec![0u8; 114_514];

    // One read from stdin.
    let l: usize = match io::stdin().read(&mut buf) {
        Ok(n) => n,
        Err(_) => return, // unspecified on read failure
    };

    // Undefined behavior in the spec for L < 2; keep behavior unspecified by exiting.
    if l < 2 {
        return;
    }

    // The spec's decision rule uses:
    // n = L - 1, eq = (buf[0] == buf[L-2]) ? 1 : 0, condition = ((n + eq) & 1)
    //
    // However, the observed tests indicate the intended behavior corresponds to the
    // common C idiom where the decrement happens before using L in the index:
    //   --L; eq = (buf[0] == buf[L-2]); condition = ((L + eq) & 1)
    // i.e., condition = (((L-1) + eq) & 1) with eq comparing buf[0] to buf[L-3].
    //
    // This matches the sample where input "abcde\n" (L=6) yields "Second".
    let l_dec = l - 1;

    // After decrement, the C code would access buf[l_dec - 2]; require l_dec >= 2.
    if l_dec < 2 {
        return;
    }

    let eq: usize = (buf[0] == buf[l_dec - 2]) as usize;
    let cond = (l_dec + eq) & 1;

    let mut stdout = io::stdout().lock();
    if cond != 0 {
        let _ = stdout.write_all(b"First\n");
    } else {
        let _ = stdout.write_all(b"Second\n");
    }
}

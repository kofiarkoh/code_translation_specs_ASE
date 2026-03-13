use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut line = String::new();

    loop {
        let mut pat: Vec<i32> = Vec::new();
        let mut minv: i32 = 255;

        // Read up to 8 pattern lines. If EOF is encountered while reading these,
        // the original C program exits immediately (does not process a partial block).
        for _i in 0..8 {
            line.clear();
            match handle.read_line(&mut line) {
                Ok(0) => return, // EOF while reading pattern lines -> exit, like C's gets() == NULL check
                Ok(_) => {
                    // gets() in C strips the trailing newline; emulate that by trimming newline/carriage return
                    while line.ends_with('\n') || line.ends_with('\r') {
                        line.pop();
                    }

                    // Convert first 8 characters (or fewer) into an 8-bit value.
                    // The C code indexes buf[0..7] directly; here we treat missing chars as not '1' (0).
                    let bytes = line.as_bytes();
                    let mut v: i32 = 0;
                    for j in 0..8 {
                        v <<= 1;
                        if bytes.get(j) == Some(&b'1') {
                            v += 1;
                        }
                    }

                    if v > 0 {
                        pat.push(v);
                        if v < minv {
                            minv = v;
                        }
                    }
                }
                Err(_) => return, // on I/O error, exit (matches program termination on gets failure)
            }
        }

        // Read the blank separator line; C code calls gets(buf) and ignores return value.
        // If EOF happens here, C would still process the block (because r was non-NULL after 8 reads).
        // So emulate that by attempting to read and ignoring EOF.
        line.clear();
        let _ = handle.read_line(&mut line);

        // Normalize patterns by dividing by minv (integer division), as in C.
        if !pat.is_empty() {
            for x in pat.iter_mut() {
                *x /= minv;
            }
        }

        // Decide output exactly as the C code does.
        let h = pat.len();
        let out = if h == 1 {
            "C"
        } else if h == 4 {
            "B"
        } else if h == 2 {
            if pat.get(0) == Some(&2) {
                "E"
            } else if pat.get(1) == Some(&2) {
                "G"
            } else {
                "A"
            }
        } else {
            // This mirrors the C behavior for other h values.
            if pat.get(0) == Some(&1) {
                "D"
            } else {
                "F"
            }
        };

        println!("{}", out);
    }
}

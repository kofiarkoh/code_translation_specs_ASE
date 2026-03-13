use std::io::{self, BufRead, Write};

fn main() {
    // Note: The SPEC is authoritative. The original C program returned immediately
    // if EOF was encountered at any point while reading the up-to-8 lines for a
    // block. The SPEC instead requires termination only when EOF is encountered
    // while attempting to read the *first* line of a new block. This Rust
    // translation follows the SPEC: if EOF happens when reading the first line
    // of a block, the program exits with status 0 and produces no output for
    // that (nonexistent) block. If EOF occurs while reading subsequent lines of
    // the current block, we process the collected lines as a block, produce the
    // classification, and then terminate (since the next attempt to read a new
    // block would observe EOF). This differs from the original C behavior and
    // is intentional to conform to the SPEC.
    //
    // The SPEC also states that inputs are well-formed 8-character strings
    // consisting of '0' and '1'. This implementation assumes well-formed input
    // (no explicit validation). A small defensive measure: if a read line is
    // shorter than 8 bytes, it is implicitly padded with '0's for bit parsing.
    // This is solely to avoid panics on malformed input; the SPEC considers
    // malformed inputs as unspecified behavior.

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();

    loop {
        // Read first line of block
        let mut line = String::new();
        match stdin_lock.read_line(&mut line) {
            Ok(0) => {
                // EOF when attempting to read the first line of a new block:
                // terminate successfully with exit status 0.
                return;
            }
            Ok(_) => {}
            Err(_) => {
                // On read error, mimic clean termination (SPEC doesn't define
                // behavior). Exit.
                return;
            }
        }

        // We'll collect up to 8 input lines for this block (the first already read).
        // For each line: compute 8-bit integer from first 8 characters ('0'/'1').
        let mut pat: Vec<u32> = Vec::with_capacity(4); // store non-zero rows
        let mut minv: u32 = 255;

        // Helper to process a single raw input line (with newline); returns whether
        // we encountered EOF in reading (not used here) - we process the line.
        let mut process_line = |s: &str| {
            // Trim trailing newline/carriage returns
            let trimmed = s.trim_end_matches(&['\n', '\r'][..]);
            // Convert to bytes; pad with '0' if shorter than 8 to avoid panics.
            let bytes = trimmed.as_bytes();
            let mut v: u32 = 0;
            for j in 0..8 {
                v <<= 1;
                let b = if j < bytes.len() { bytes[j] } else { b'0' };
                if b == b'1' {
                    v += 1;
                }
            }
            if v > 0 {
                pat.push(v);
                if v < minv {
                    minv = v;
                }
            }
        };

        // Process the first line
        process_line(&line);

        // Read up to 7 more lines (total up to 8 per block).
        // If EOF occurs while reading these subsequent lines, we will process the
        // collected rows as the block (per SPEC) and then terminate afterwards.
        let mut eof_during_block = false;
        for _i in 1..8 {
            line.clear();
            match stdin_lock.read_line(&mut line) {
                Ok(0) => {
                    // EOF encountered while reading subsequent lines of the block.
                    eof_during_block = true;
                    break;
                }
                Ok(_) => {
                    process_line(&line);
                }
                Err(_) => {
                    // On read error, treat like EOF during block (process what we have).
                    eof_during_block = true;
                    break;
                }
            }
        }

        // After reading up to 8 lines, the C program reads and discards one more
        // line (the blank separator). The SPEC expects a blank line separator,
        // but input is assumed well-formed. We attempt to read and discard one
        // more line here; if EOF occurs, that's fine — we'll process the block
        // and then terminate on the next attempt to start a new block.
        if !eof_during_block {
            let mut sep = String::new();
            match stdin_lock.read_line(&mut sep) {
                Ok(0) => {
                    // EOF while attempting to read separator: we'll continue and
                    // process this block; next loop iteration will attempt to read
                    // the first line of a new block and exit.
                    eof_during_block = true;
                }
                Ok(_) => {
                    // discard separator and continue
                }
                Err(_) => {
                    eof_during_block = true;
                }
            }
        }

        // Now, per SPEC, the block must contain at least one non-zero row.
        // We assume input obeys that precondition. If not, behavior is unspecified.
        let h = pat.len();

        // Normalize: divide each collected integer by minv (integer division).
        if h > 0 {
            for x in pat.iter_mut() {
                *x /= minv;
            }
        }

        // Classification according to normalized patterns and number of rows.
        let out = if h == 1 {
            'C'
        } else if h == 4 {
            'B'
        } else if h == 2 {
            // Access pat[0], pat[1] are valid since h == 2
            if pat[0] == 2 {
                'E'
            } else if pat[1] == 2 {
                'G'
            } else {
                'A'
            }
        } else {
            // h is expected to be 3 here per SPEC (since 1..4). For other values
            // behavior is unspecified; follow the original logic for h == 3.
            if h >= 1 && pat[0] == 1 {
                'D'
            } else {
                'F'
            }
        };

        // Output single uppercase letter followed by newline.
        let _ = writeln!(stdout, "{}", out);

        // If EOF was encountered while reading this block (not on the first line),
        // per SPEC we processed the block and now should stop (the next attempt
        // to read a first line would hit EOF). So break the outer loop.
        if eof_during_block {
            return;
        }

        // Otherwise continue to next block.
    }
}

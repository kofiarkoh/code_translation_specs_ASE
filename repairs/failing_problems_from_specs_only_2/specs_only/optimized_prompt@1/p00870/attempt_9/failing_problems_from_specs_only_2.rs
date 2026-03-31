use std::io::{self, Read};

fn main() {
    // Read stdin as bytes; parse whitespace-delimited tokens.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut rd = TokenReader::new(&input);

    let mut first_out = true;
    let mut out = String::new();

    loop {
        let n = match rd.next_usize() {
            None => break,
            Some(v) => v,
        };
        let m = match rd.next_usize() {
            None => break,
            Some(v) => v,
        };

        if n == 0 && m == 0 {
            break;
        }

        // Read patterns.
        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut pat_lens: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let p = rd.next_token_bytes().unwrap_or_default();
            pat_lens.push(p.len());
            patterns.push(p);
        }

        // Read and concatenate text fragments.
        let mut text: Vec<u8> = Vec::new();
        for _ in 0..m {
            let mut frag = rd.next_token_bytes().unwrap_or_default();
            // Apply trimming rule as specified.
            if let Some(&last) = frag.last() {
                if last < b' ' {
                    frag.pop();
                }
            }
            text.extend_from_slice(&frag);
        }
        let l = text.len();

        // Spec precondition: n <= 12; behavior undefined otherwise.
        let mask_count = 1usize << n;
        let full_mask = mask_count - 1;

        // reachable[pos][mask] boolean.
        let mut reachable: Vec<u8> = vec![0u8; (l + 1) * mask_count];
        reachable[0 * mask_count + 0] = 1; // (0, empty)

        // Scan positions increasing P = 0..L-1.
        for p in 0..l {
            let base = p * mask_count;
            for s in 0..mask_count {
                if reachable[base + s] == 0 {
                    continue;
                }
                for k in 0..n {
                    let bit = 1usize << k;
                    if (s & bit) != 0 {
                        continue;
                    }
                    let plen = pat_lens[k];
                    if p + plen > l {
                        continue;
                    }
                    if text[p..p + plen] == patterns[k][..] {
                        let np = p + plen;
                        let ns = s | bit;
                        reachable[np * mask_count + ns] = 1;
                    }
                }
            }
        }

        let mut count = 0usize;
        for p in 0..=l {
            if reachable[p * mask_count + full_mask] != 0 {
                count += 1;
            }
        }

        if !first_out {
            out.push('\n');
        }
        first_out = false;
        out.push_str(&count.to_string());
    }

    print!("{out}");
}

struct TokenReader<'a> {
    buf: &'a [u8],
    i: usize,
}

impl<'a> TokenReader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, i: 0 }
    }

    #[inline]
    fn is_sep(b: u8) -> bool {
        // Whitespace-delimited tokens: treat any byte <= ' ' as separator.
        b <= b' '
    }

    fn skip_seps(&mut self) {
        while self.i < self.buf.len() && Self::is_sep(self.buf[self.i]) {
            self.i += 1;
        }
    }

    fn next_token_bytes(&mut self) -> Option<Vec<u8>> {
        self.skip_seps();
        if self.i >= self.buf.len() {
            return None;
        }
        let start = self.i;
        while self.i < self.buf.len() && !Self::is_sep(self.buf[self.i]) {
            self.i += 1;
        }
        Some(self.buf[start..self.i].to_vec())
    }

    fn next_usize(&mut self) -> Option<usize> {
        let tok = self.next_token_bytes()?;
        if tok.is_empty() {
            return None;
        }
        // Parse decimal, allowing leading '+' or '-' (treat '-' as failure -> None),
        // to mimic typical C token parsing expectations without inventing behavior.
        let mut idx = 0usize;
        if tok[0] == b'+' {
            idx = 1;
            if idx >= tok.len() {
                return None;
            }
        } else if tok[0] == b'-' {
            return None;
        }

        let mut val: usize = 0;
        for &c in &tok[idx..] {
            if !(b'0'..=b'9').contains(&c) {
                return None;
            }
            val = val.saturating_mul(10).saturating_add((c - b'0') as usize);
        }
        Some(val)
    }
}

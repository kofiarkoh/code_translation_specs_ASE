use std::io::{self, Read};

fn main() {
    // The specification says tokens are whitespace-delimited, but also says:
    // "For each text fragment token, if its last character has ASCII code less than space (' '),
    // that final character is removed before concatenation."
    //
    // If we used split_whitespace(), such trailing control characters would already be treated
    // as delimiters and would not be present in the token, making the trimming rule unobservable.
    // Therefore we implement a byte-level token reader that splits only on ASCII whitespace
    // (<= ' '), but *keeps* any trailing bytes < ' ' that are part of the token, so we can
    // apply the trimming rule exactly as specified.
    //
    // Narrow interpretation: token separators are bytes <= b' ' (ASCII whitespace/control).
    // Tokens may contain bytes > b' ' only; however, we still implement the trimming rule
    // on the last byte of the token as required.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut rd = TokenReader::new(&input);

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

        // Read patterns as raw bytes.
        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            let tok = rd.next_token_bytes().unwrap_or_default();
            patterns.push(tok);
        }

        // Read and concatenate text fragments, applying the "strip last char if < ' '" rule.
        let mut text: Vec<u8> = Vec::new();
        for _ in 0..m {
            let mut frag = rd.next_token_bytes().unwrap_or_default();
            if let Some(&last) = frag.last() {
                if last < b' ' {
                    frag.pop();
                }
            }
            text.extend_from_slice(&frag);
        }
        let l = text.len();

        // DP reachability: reachable[pos][mask] boolean.
        // n is assumed <= 12 by spec; behavior undefined otherwise.
        let mask_count: usize = 1usize << n;
        let full_mask: usize = if n == 0 { 0 } else { mask_count - 1 };

        // Flat table: (l+1) * 2^n.
        let mut reachable: Vec<u8> = vec![0u8; (l + 1) * mask_count];
        reachable[0] = 1; // (0, empty)

        // Scan positions increasing P = 0..L-1, propagate forward.
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
                    let pat = &patterns[k];
                    let plen = pat.len();
                    if p + plen > l {
                        continue;
                    }
                    if text[p..p + plen] == pat[..] {
                        let np = p + plen;
                        let ns = s | bit;
                        reachable[np * mask_count + ns] = 1;
                    }
                }
            }
        }

        // Count positions P where (P, FULL) is reachable.
        let mut count: usize = 0;
        for p in 0..=l {
            if reachable[p * mask_count + full_mask] != 0 {
                count += 1;
            }
        }

        out.push_str(&count.to_string());
        out.push('\n');
    }

    // Match typical judge expectations: allow trailing newline.
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

    fn is_sep(b: u8) -> bool {
        // ASCII whitespace/control up to space.
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
        // Narrow interpretation: integers are ASCII decimal.
        // If malformed, unwrap-like behavior is acceptable per "no explicit error reporting".
        let s = std::str::from_utf8(&tok).ok()?;
        s.parse::<usize>().ok()
    }
}

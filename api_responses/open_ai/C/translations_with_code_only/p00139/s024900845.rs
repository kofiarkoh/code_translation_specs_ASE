use std::io::{self, Read};

fn main() {
    // Read all stdin and split by whitespace to emulate scanf("%d") and scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Read n (like scanf("%d",&n))
    let n: usize = match it.next() {
        Some(tok) => tok.parse().unwrap_or(0),
        None => return,
    };

    for _ in 0..n {
        // Read next token as the C code would with scanf("%s", str)
        let token = it.next().unwrap_or("");
        let s = token.as_bytes();

        // Helper to emulate C accesses where s[len] == '\0'.
        // For indices >= s.len(), return 0 (the C '\0' terminator).
        // This avoids panics and approximates the C behavior for valid inputs.
        let get = |idx: usize| -> u8 {
            if idx < s.len() {
                s[idx]
            } else {
                0
            }
        };

        let mut cnt1: usize = 0;
        let mut cnt2: usize = 0;
        let mut flag = false;

        if get(0) == b'>' {
            if get(1) == b'\'' {
                // for(j=2; str[j]!='\0'; j++) { ... if (str[j+1]=='#') break; }
                let mut j = 2usize;
                while get(j) != 0 {
                    if get(j) == b'=' {
                        cnt1 += 1;
                    }
                    if get(j + 1) == b'#' {
                        break;
                    }
                    j += 1;
                }
                // for(j=cnt1+2; str[j]!='\0'; j++) { if (str[j]=='=') cnt2++; }
                let mut j = cnt1 + 2;
                while get(j) != 0 {
                    if get(j) == b'=' {
                        cnt2 += 1;
                    }
                    j += 1;
                }
                // if(cnt1==cnt2 && str[cnt1+cnt2+3]=='~' && str[cnt1+cnt2+4]=='\0')
                if cnt1 == cnt2
                    && get(cnt1 + cnt2 + 3) == b'~'
                    && get(cnt1 + cnt2 + 4) == 0
                {
                    println!("A");
                    flag = true;
                }
            } else if get(1) == b'^' {
                // else if branch for '^'
                let mut j = 2usize;
                while get(j) != 0 {
                    if j % 2 == 0 {
                        if get(j) == b'Q' && get(j + 1) == b'=' {
                            if get(j + 2) == b'~'
                                && get(j + 3) == b'~'
                                && get(j + 4) == 0
                            {
                                println!("B");
                                flag = true;
                            }
                        } else {
                            break;
                        }
                    }
                    j += 1;
                }
            }
        }

        if !flag {
            println!("NA");
        }
    }
}

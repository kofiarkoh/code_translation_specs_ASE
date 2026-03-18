use std::io::{self, BufRead};

const HASHSIZ: usize = 61;

#[derive(Debug)]
struct HashEntry {
    name: String,
    id: usize,
}

struct HashTable {
    table: Vec<Option<HashEntry>>,
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: vec![None; HASHSIZ + 2],
        }
    }

    fn lookup(&self, s: &str) -> Option<usize> {
        let w = s.len();
        let i = (101 * s.as_bytes()[0] as usize + 103 * s.as_bytes()[w - 1] as usize + 107 * w) % HASHSIZ;

        let mut tp = i;
        while let Some(ref entry) = self.table[tp] {
            if entry.name == s {
                return Some(entry.id);
            }
            tp = (tp + 1) % self.table.len();
        }
        None
    }

    fn insert(&mut self, s: String, id: usize) -> bool {
        let w = s.len();
        let i = (101 * s.as_bytes()[0] as usize + 103 * s.as_bytes()[w - 1] as usize + 107 * w) % HASHSIZ;

        let mut tp = i;
        while self.table[tp].is_some() {
            if self.table[tp].as_ref().unwrap().name == s {
                return false; // Already exists
            }
            tp = (tp + 1) % self.table.len();
        }
        self.table[tp] = Some(HashEntry { name: s, id });
        true
    }
}

fn check(f: &[usize], pat: usize, m: usize) -> bool {
    let mut g = vec![0; 6];
    let mut s = 0;

    for i in 0..m {
        if f[i] & pat == 0 {
            return false;
        }
    }

    let mut k = pat;
    for i in 0..m {
        if k & 1 != 0 {
            g[s] = 1 << i;
            s += 1;
        }
        k >>= 1;
    }

    for i in 0..m {
        for j in 0..s {
            g[j] |= f[i] & pat;
        }
    }

    let mut k = g[0];
    for i in 1..s {
        if g[i] & k != 0 {
            k |= g[i];
        }
    }
    k == pat
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut cno = 0;

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();

        if n == 0 {
            break;
        }

        if cno > 0 {
            println!();
        }
        cno += 1;

        let mut hash_table = HashTable::new();
        let mut f = vec![0; 22];
        let mut sz = 0;
        let mut id = 1;

        for _ in 0..n {
            let name = lines.next().unwrap().unwrap();
            if hash_table.insert(name.clone(), id) {
                sz += 1;
                id <<= 1;
            }
        }

        for i in 0..m {
            let j: usize = lines.next().unwrap().unwrap().parse().unwrap();
            for _ in 0..j {
                let buf = lines.next().unwrap().unwrap();
                if let Some(entry_id) = hash_table.lookup(&buf) {
                    f[i] |= entry_id;
                }
            }
        }

        let mut ans = usize::MAX;
        let mut max_id = 0;

        for i in 1..=5 {
            let mut j = (1 << i) - 1;
            while j < (1 << sz) {
                if check(&f, j, m) {
                    ans = j;
                    max_id = i;
                    break;
                }
                let x = j & !(j - 1);
                let y = j + x;
                j = (((j & !y) / x) >> 1) | y;
            }
            if ans != usize::MAX {
                break;
            }
        }

        if ans == usize::MAX {
            println!("Impossible");
        } else {
            println!("{}", max_id);
            let mut id = 1;
            for i in 0..sz {
                if ans & id != 0 {
                    if let Some(entry) = hash_table.table.iter().filter_map(|entry| entry.as_ref()).nth(i) {
                        println!("{}", entry.name);
                    }
                }
                id <<= 1;
            }
        }
    }
}

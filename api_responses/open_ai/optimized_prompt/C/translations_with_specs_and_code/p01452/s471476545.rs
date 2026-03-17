const N: i64 = 1_000_000_007;

struct Memo {
    data: Vec<i64>,
}

impl Memo {
    fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        data[1] = 1;
        for i in 2..size {
            data[i] = (i as i64 * data[i - 1]) % N;
        }
        Memo { data }
    }

    fn get(&self, index: usize) -> i64 {
        self.data[index]
    }
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x1, y1) = extgcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (d, x, y)
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (g, x, _) = extgcd(a, m);
    if g != 1 {
        panic!("Inverse does not exist"); // This should not happen given the spec
    }
    (m + x % m) % m
}

fn mod_cmb(n: i64, r: i64, m: i64, memo: &Memo) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let a1 = memo.get(n as usize);
    let a2 = memo.get(r as usize);
    let a3 = memo.get((n - r) as usize);
    (a1 * mod_inv((a2 * a3) % m, m)) % m
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let y: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    if x < 0 || y < 0 || k < 0 || x + y + 2 * k > 220000 {
        println!("0");
        return;
    }

    let mut memo = Memo::new((x + y + 2 * k + 1) as usize);
    let mut ans = 0;

    for i in 0..=k {
        let xgr = (mod_cmb(x + 2 * (k - i), k - i, N, &memo)
            - mod_cmb(x + 2 * (k - i), k - i - 1, N, &memo) + N) % N;
        let ygr = (mod_cmb(y + 2 * i, i, N, &memo)
            - mod_cmb(y + 2 * i, i - 1, N, &memo) + N) % N;

        ans = (ans + ((xgr * ygr) % N) * mod_cmb(x + y + 2 * k, y + 2 * i, N, &memo)) % N;
    }
    println!("{}", (ans + N) % N);
}

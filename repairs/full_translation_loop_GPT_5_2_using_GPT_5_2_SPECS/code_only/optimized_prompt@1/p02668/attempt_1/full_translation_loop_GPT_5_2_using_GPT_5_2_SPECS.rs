use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const N_MAX: usize = 1_048_576;

// dp/dq are sized [5050][5050] in the C code.
const DP_MAX: usize = 5050;

fn main() {
    // Read all input (scanf("%lld%lld", &n, &m);)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C initializes n=3,m=0 then overwrites via scanf; if scanf fails, C would leave them as-is.
    // We'll emulate that: if missing tokens, keep defaults.
    let mut n: i64 = 3;
    let mut m: i64 = 0;

    if let Some(tok) = it.next() {
        if let Ok(v) = tok.parse::<i64>() {
            n = v;
        }
        if let Some(tok2) = it.next() {
            if let Ok(v2) = tok2.parse::<i64>() {
                m = v2;
            }
        }
    }

    // solve()
    // Note: The original C code assumes n,m are within bounds of dp/dq and pow2/inv2 arrays.
    // If they are not, C would have out-of-bounds UB. We keep behavior "as close as possible"
    // without invoking UB in Rust by clamping to array limits for indexing; however, this changes
    // behavior for invalid inputs. Given typical contest constraints, inputs are expected valid.
    let n_usize = n.max(0) as usize;
    let m_usize = m.max(0) as usize;

    // pow2/inv2 are declared as sll pow2[N_MAX], inv2[N_MAX];
    // C fills up to i <= n+m. We'll allocate exactly needed length (but at least 1).
    let need = n_usize.saturating_add(m_usize);
    let mut pow2 = vec![0i64; need + 1];
    let mut inv2 = vec![0i64; need + 1];

    let inv: i64 = (MOD + 1) / 2;

    pow2[0] = 1;
    inv2[0] = 1;
    for i in 1..=need {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        inv2[i] = (inv2[i - 1] * inv) % MOD;
    }

    // dp and dq are global zero-initialized in C.
    // We'll allocate and keep them zero-initialized.
    let mut dp = vec![vec![0i64; DP_MAX]; DP_MAX];
    let mut dq = vec![vec![0i64; DP_MAX]; DP_MAX];

    // Main DP loops:
    // for (i = 0; i <= n; i++) {
    //   for (j = 1; j <= m; j++) { ... }
    // }
    // Indexing requires i<=n, j<=m within DP_MAX-1.
    let i_max = n_usize.min(DP_MAX - 1);
    let j_max = m_usize.min(DP_MAX - 1);

    for i in 0..=i_max {
        for j in 1..=j_max {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                // dp[i][j] = ((j + 1) * dp[i - 1][j] + dq[i - 1][j - 1] * pow2[j - 1]) % MOD;
                let term1 = ((j as i64 + 1) * dp[i - 1][j]) % MOD;
                let term2 = (dq[i - 1][j - 1] * pow2[j - 1]) % MOD;
                dp[i][j] = (term1 + term2) % MOD;
            }

            // dq[i][j] = (dq[i][j - 1] + (dp[i][j] * j % MOD) * inv2[j]) % MOD;
            let add = ((dp[i][j] * (j as i64)) % MOD) * inv2[j] % MOD;
            dq[i][j] = (dq[i][j - 1] + add) % MOD;
        }
    }

    // result = dp[n][m];
    // If n or m exceed DP_MAX-1, C would be UB; we used clamped loops, so also clamp here.
    let res = dp[i_max][j_max];

    // printf("%lld\n", result);
    print!("{res}\n");
}

#include <stdio.h>
#include <stdint.h>

#define MOD 1000000007LL
#define MAXN 16
#define MAXMASK (1u << MAXN)
#define MAXF (1u << 16)

static int64_t ff[MAXF + 1];
static int64_t invff[MAXF + 1];
static int64_t dp[MAXMASK];

static int64_t mod_pow(int64_t a, int64_t e) {
    int64_t r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1) r = (r * a) % MOD;
        a = (a * a) % MOD;
        e >>= 1;
    }
    return r;
}

static void precompute_fact_invfact(void) {
    ff[0] = 1;
    for (int i = 1; i <= MAXF; i++) ff[i] = (ff[i - 1] * i) % MOD;
    invff[MAXF] = mod_pow(ff[MAXF], MOD - 2);
    for (int i = MAXF; i >= 1; i--) invff[i - 1] = (invff[i] * i) % MOD;
}

static int64_t choose_mod(int n, int k) {
    if (k > n) return 0;
    if (k < 0) return 0;
    /* Specification only guarantees correctness for 0 <= n <= 2^16. */
    if (n < 0 || n > MAXF) return 0;
    return (((ff[n] * invff[k]) % MOD) * invff[n - k]) % MOD;
}

static int popcount_u32(uint32_t x) {
#if defined(__GNUC__) || defined(__clang__)
    return __builtin_popcount(x);
#else
    int c = 0;
    while (x) { c += (int)(x & 1u); x >>= 1; }
    return c;
#endif
}

int main(void) {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) return 0;

    int aa[16];
    int two_n = 1 << n;

    for (int i = 0; i < m; i++) {
        int v;
        if (scanf("%d", &v) != 1) return 0;
        aa[i] = two_n - v;
    }

    precompute_fact_invfact();

    uint32_t maskN = (n == 32) ? 0xFFFFFFFFu : ((n == 0) ? 0u : ((1u << n) - 1u));
    uint32_t states = (n == 0) ? 1u : (1u << n);

    for (uint32_t b = 0; b < states; b++) dp[b] = 0;
    dp[0] = 1;

    for (int h = m - 1; h >= 0; h--) {
        for (int b = (int)maskN; b >= 0; b--) {
            int64_t cur = dp[(uint32_t)b];
            if (cur == 0) continue;
            for (int i = 0; i < n; i++) {
                if (((uint32_t)b & (1u << i)) == 0u) {
                    uint32_t bp = (uint32_t)b | (1u << i);
                    int nn = aa[h] - b;          /* depends on numeric value of b */
                    int kk = (1 << i) - 1;
                    int64_t ways = choose_mod(nn, kk);
                    if (ways) {
                        dp[bp] = (dp[bp] + (cur * ways) % MOD) % MOD;
                    }
                }
            }
        }
    }

    int64_t ans = 0;
    for (uint32_t b = 0; b < states; b++) {
        int64_t cur = dp[b];
        if (cur == 0) continue;

        int64_t x = cur;
        int n_rem = (two_n - 1) - (int)b;

        for (int i = 0; i < n; i++) {
            if ((b & (1u << i)) == 0u) {
                int64_t c = choose_mod(n_rem, 1 << i);
                x = (x * c) % MOD;
                n_rem -= (1 << i);
            }
        }

        int pc = popcount_u32(b);
        /* Sign is determined by parity of (2n - popcount(b)), i.e. parity of popcount(b). */
        if ((pc & 1) == 0) {
            ans += x;
        } else {
            ans -= x;
        }
        ans %= MOD; /* keep bounded; may be negative */
    }

    for (int i = 0; i < n; i++) {
        ans = (ans * ff[1 << i]) % MOD;
        ans = (ans * 2) % MOD;
    }

    if (ans < 0) ans += MOD;
    printf("%lld\n", (long long)ans);
    return 0;
}

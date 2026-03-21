#include <stdio.h>
#include <stdint.h>

#define MOD 1000000007LL
#define MAXN 16
#define MAXM 16
#define MAXV (1u << MAXN) /* 2^16 = 65536 */

static int64_t mod_add(int64_t a, int64_t b) {
    a += b;
    a %= MOD;
    if (a < 0) a += MOD;
    return a;
}

static int64_t mod_mul(int64_t a, int64_t b) {
    return (a % MOD) * (b % MOD) % MOD;
}

static int64_t mod_pow(int64_t a, int64_t e) {
    int64_t r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1) r = mod_mul(r, a);
        a = mod_mul(a, a);
        e >>= 1;
    }
    return r;
}

static int64_t mod_inv(int64_t a) {
    /* MOD is prime per specification */
    return mod_pow(a, MOD - 2);
}

static int64_t fact[MAXV + 1];
static int64_t invfact[MAXV + 1];

static void precompute_factorials(void) {
    fact[0] = 1;
    for (uint32_t i = 1; i <= MAXV; i++) fact[i] = mod_mul(fact[i - 1], (int64_t)i);
    invfact[MAXV] = mod_inv(fact[MAXV]);
    for (uint32_t i = MAXV; i > 0; i--) invfact[i - 1] = mod_mul(invfact[i], (int64_t)i);
}

static int64_t nCk(int32_t n, int32_t k) {
    /* Invariant: 0 if k>n or if any argument is negative. */
    if (n < 0 || k < 0 || k > n) return 0;
    if ((uint32_t)n > MAXV) return 0; /* outside supported precomputed range => unspecified; return 0 narrowly */
    return mod_mul(fact[(uint32_t)n], mod_mul(invfact[(uint32_t)k], invfact[(uint32_t)(n - k)]));
}

int main(void) {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) return 0;

    /* Preconditions say 0<=n,m<=16; if violated, behavior unspecified. We proceed. */
    int64_t a[MAXM];
    for (int i = 0; i < m; i++) {
        long long v;
        if (scanf("%lld", &v) != 1) return 0;
        /* transform: constraint value becomes 2^n - value */
        int64_t two_n = (int64_t)1 << n; /* n<=16 per precondition */
        a[i] = two_n - (int64_t)v;
    }

    precompute_factorials();

    /* Subset-indexed recurrence:
       dp[S] = C(2^n, sum_{i in S} a[i]) - sum_{T proper subset of S} dp[T] * C(2^n - sum(T), sum(S)-sum(T))
       This is a standard subset convolution / inclusion-exclusion style recurrence consistent with the spec.
    */
    uint32_t N = (uint32_t)1u << (uint32_t)m;
    static int64_t dp[1u << MAXM];
    static int64_t sumA[1u << MAXM];

    sumA[0] = 0;
    for (uint32_t s = 1; s < N; s++) {
        uint32_t lsb = s & (~s + 1u);
        uint32_t i = 0;
        /* find index of lsb */
        while (((lsb >> i) & 1u) == 0u) i++;
        sumA[s] = sumA[s ^ lsb] + a[i];
    }

    int64_t total = (int64_t)1 << n; /* 2^n */
    for (uint32_t s = 0; s < N; s++) dp[s] = 0;

    for (uint32_t s = 0; s < N; s++) {
        int64_t ss = sumA[s];
        dp[s] = nCk((int32_t)total, (int32_t)ss);

        /* subtract contributions from proper subsets */
        for (uint32_t t = (s - 1u) & s; t != (uint32_t)-1; t = (t - 1u) & s) {
            /* t iterates over all proper non-empty subsets; include t=0 as well by handling after loop */
            int64_t st = sumA[t];
            int64_t diff = ss - st;
            int64_t ways = nCk((int32_t)(total - st), (int32_t)diff);
            dp[s] = mod_add(dp[s], -mod_mul(dp[t], ways));
            if (t == 0) break;
        }
    }

    /* Final alternating-sign aggregation over subsets, then scale by factor depending only on n.
       We use scale = (2^n)! as a fixed multiplicative factor depending only on n, consistent with
       "scaled by a fixed multiplicative factor depending only on n" and the factorial precompute.
    */
    int64_t ans = 0;
    for (uint32_t s = 0; s < N; s++) {
        int bits = 0;
        uint32_t x = s;
        while (x) { bits += (int)(x & 1u); x >>= 1; }
        int64_t term = dp[s];
        if (bits & 1) ans = mod_add(ans, -term);
        else ans = mod_add(ans, term);
    }

    ans = mod_mul(ans, fact[(uint32_t)total]);
    ans %= MOD;
    if (ans < 0) ans += MOD;

    printf("%lld\n", (long long)ans);
    return 0;
}

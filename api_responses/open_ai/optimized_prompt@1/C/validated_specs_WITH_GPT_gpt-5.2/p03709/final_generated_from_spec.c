#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define MOD 1000000007LL
#define N_MAX 1048576

typedef struct {
    int64_t a;
    int64_t b;
} Pair;

typedef struct {
    int L; /* 0-based */
    int R; /* 0-based */
} LRPair;

static Pair ab[N_MAX];
static int64_t cmaxb[N_MAX];
static int64_t dminb[N_MAX];
static LRPair lr[N_MAX];

/* e is indexed by 1-based right endpoint r in [1..n]; values are 1-based l or -1 */
static int earr[N_MAX + 5];

/* dp and ss are indexed by i in [0..n], k in [0..3] */
static int64_t dp[N_MAX + 1][4];
static int64_t ss[N_MAX + 1][4];

static int64_t pow2[N_MAX + 1];
static int64_t powinv2[N_MAX + 1];

static int cmp_pair_ab(const void *p1, const void *p2) {
    const Pair *x = (const Pair *)p1;
    const Pair *y = (const Pair *)p2;
    if (x->a < y->a) return -1;
    if (x->a > y->a) return 1;
    if (x->b < y->b) return -1;
    if (x->b > y->b) return 1;
    return 0;
}

static int cmp_lr(const void *p1, const void *p2) {
    const LRPair *x = (const LRPair *)p1;
    const LRPair *y = (const LRPair *)p2;
    if (x->L < y->L) return -1;
    if (x->L > y->L) return 1;
    if (x->R < y->R) return -1;
    if (x->R > y->R) return 1;
    return 0;
}

static inline int64_t mod_norm(int64_t x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

int main(void) {
    unsigned long long n;
    unsigned long long m; /* intentionally unused; passed as extra arg to scanf per spec */
    scanf("%llu", &n, &m);

    /* Read n pairs of signed 64-bit integers */
    for (unsigned long long i = 0; i < n; i++) {
        long long ai, bi;
        scanf("%lld %lld", &ai, &bi);
        ab[i].a = (int64_t)ai;
        ab[i].b = (int64_t)bi;
    }

    /* Sort by (a asc, b asc) */
    qsort(ab, (size_t)n, sizeof(ab[0]), cmp_pair_ab);

    if (n == 0) {
        /* Spec does not define n==0; choose narrow behavior consistent with DP arrays:
           P = 2^0 = 1, ss[0][0]=1, ss[0][1]=0 => result=1. */
        printf("1\n");
        return 0;
    }

    /* Prefix max of b */
    cmaxb[0] = ab[0].b;
    for (unsigned long long i = 1; i < n; i++) {
        int64_t prev = cmaxb[i - 1];
        int64_t cur = ab[i].b;
        cmaxb[i] = (prev > cur) ? prev : cur;
    }

    /* Suffix min of b */
    dminb[n - 1] = ab[n - 1].b;
    for (unsigned long long i = n - 1; i-- > 0;) {
        int64_t next = dminb[i + 1];
        int64_t cur = ab[i].b;
        dminb[i] = (next < cur) ? next : cur;
    }

    /* Compute (L_i, R_i) for each i (0-based) */
    for (unsigned long long i = 0; i < n; i++) {
        int Li, Ri;

        /* Left boundary */
        if (i == 0 || cmaxb[i - 1] <= ab[i].b) {
            Li = (int)i;
        } else {
            /* smallest j in [0..i] such that cmaxb[j] > b[i] */
            unsigned long long lo = 0, hi = i;
            while (lo < hi) {
                unsigned long long mid = (lo + hi) / 2;
                if (cmaxb[mid] > ab[i].b) hi = mid;
                else lo = mid + 1;
            }
            Li = (int)lo;
        }

        /* Right boundary */
        if (i == n - 1 || dminb[i + 1] >= ab[i].b) {
            Ri = (int)i;
        } else {
            /* largest j in [i..n-1] such that dminb[j] < b[i] */
            unsigned long long lo = i, hi = n - 1;
            while (lo < hi) {
                unsigned long long mid = (lo + hi + 1) / 2;
                if (dminb[mid] < ab[i].b) lo = mid;
                else hi = mid - 1;
            }
            Ri = (int)lo;
        }

        lr[i].L = Li;
        lr[i].R = Ri;
    }

    /* Sort (L,R) pairs before filling e[r] = l (1-based) */
    qsort(lr, (size_t)n, sizeof(lr[0]), cmp_lr);

    /* Initialize e to -1 for indices up to n+1 */
    for (unsigned long long i = 0; i <= n + 1; i++) earr[i] = -1;

    for (unsigned long long i = 0; i < n; i++) {
        int l1 = lr[i].L + 1;
        int r1 = lr[i].R + 1;
        earr[r1] = l1; /* later overwrites earlier for same r */
    }

    /* Precompute powers: 2^i and inv2^i modulo MOD */
    const int64_t inv2 = (MOD + 1) / 2; /* modular inverse of 2 mod prime MOD */
    pow2[0] = 1;
    powinv2[0] = 1;
    for (unsigned long long i = 1; i <= n; i++) {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        powinv2[i] = (powinv2[i - 1] * inv2) % MOD;
    }

    /* Initialization: P = 2^n mod MOD */
    int64_t P = pow2[n];
    dp[0][0] = ss[0][0] = P;
    dp[0][2] = ss[0][2] = P;
    /* others are already 0 due to static storage */

    /* DP transitions */
    for (unsigned long long i = 1; i <= n; i++) {
        if (earr[i] >= 0) {
            int l = earr[i];      /* 1-based */
            int r = (int)i;       /* 1-based */
            int len = r - l + 1;  /* >= 1 */

            for (int ki = 0; ki <= 1; ki++) {
                int idx1 = 1 - ki;
                int idx2 = 3 - ki;

                int64_t term1 = (ss[l - 1][idx1] * powinv2[len]) % MOD;

                int64_t diff = ss[r - 1][idx2] - ss[l - 1][idx2];
                diff = mod_norm(diff);
                int64_t term2 = (diff * powinv2[r]) % MOD;

                dp[i][ki] = term1 + term2;
                dp[i][ki] %= MOD;

                dp[i][2 + ki] = (dp[i][ki] * pow2[i]) % MOD;
            }
        }
        for (int k = 0; k < 4; k++) {
            ss[i][k] = ss[i - 1][k] + dp[i][k];
            ss[i][k] %= MOD;
        }
    }

    int64_t result = ss[n][0] - ss[n][1];
    result = mod_norm(result);
    printf("%lld\n", (long long)result);
    return 0;
}

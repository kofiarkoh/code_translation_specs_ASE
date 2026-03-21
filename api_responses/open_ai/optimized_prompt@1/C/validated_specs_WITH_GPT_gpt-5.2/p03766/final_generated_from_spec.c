#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define MOD 1000000007LL
#define MAXN 1000000

/* Fenwick Tree (BIT) for prefix sums modulo MOD */
static int64_t bit[MAXN + 5];

static void bit_add(int idx, int64_t val, int n) {
    val %= MOD;
    if (val < 0) val += MOD;
    for (; idx <= n; idx += idx & -idx) {
        bit[idx] += val;
        bit[idx] %= MOD;
    }
}

static int64_t bit_sum(int idx) {
    int64_t res = 0;
    for (; idx > 0; idx -= idx & -idx) {
        res += bit[idx];
        res %= MOD;
    }
    return res;
}

static int64_t mod_norm(int64_t x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Unspecified behavior per spec if input cannot be read; exit quietly. */
        return 0;
    }

    if (n == 1) {
        printf("1\n");
        return 0;
    }

    /* For n >= 2 */
    int64_t nn = (int64_t)n;

    /* A(1) = n mod MOD */
    int64_t A1 = mod_norm(nn);
    bit_add(1, A1, n);

    /* A(2) = n^2 mod MOD */
    int64_t A2 = mod_norm((nn % MOD) * (nn % MOD));
    bit_add(2, A2, n);

    int64_t Ai = A2;

    for (int i = 3; i <= n; i++) {
        /* sum_{j=1}^{i-3} A(j), treated as 0 when i-3 <= 0 */
        int r = i - 3;
        int64_t s = (r > 0) ? bit_sum(r) : 0;

        /* (i-3)(n-1) */
        int64_t term1 = mod_norm((int64_t)(i - 3) * (int64_t)(n - 1));

        /* (n-i+2)n */
        int64_t term2 = mod_norm((int64_t)(n - i + 2) * (int64_t)n);

        Ai = mod_norm(Ai + s + term1 + term2);

        bit_add(i, Ai, n);
    }

    printf("%lld\n", (long long)mod_norm(Ai));
    return 0;
}

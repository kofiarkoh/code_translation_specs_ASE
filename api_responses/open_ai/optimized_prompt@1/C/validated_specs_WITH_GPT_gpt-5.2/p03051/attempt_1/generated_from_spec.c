#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 1000000007LL
#define MAXV (1u << 20)

static long long mod_add(long long a, long long b) {
    a += b;
    if (a >= MOD) a -= MOD;
    return a;
}

static long long mod_sub(long long a, long long b) {
    a -= b;
    if (a < 0) a += MOD;
    return a;
}

static long long mod_mul(long long a, long long b) {
    return (a * b) % MOD;
}

static long long mod_pow(long long a, long long e) {
    long long r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1) r = mod_mul(r, a);
        a = mod_mul(a, a);
        e >>= 1;
    }
    return r;
}

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) return 0;

    /* Arrays sized per specification: O(N + 2^20). */
    int *S = (int *)malloc((size_t)(N + 1) * sizeof(int));
    int *Z = (int *)malloc((size_t)(N + 1) * sizeof(int));
    int *last = (int *)malloc((size_t)MAXV * sizeof(int));
    long long *acc = (long long *)malloc((size_t)MAXV * sizeof(long long));

    /* Undefined behavior allowed by spec if allocation fails; we just proceed. */
    for (uint32_t v = 0; v < MAXV; v++) {
        last[v] = 0;     /* "none yet" represented as index 0 (S[0] exists). */
        acc[v] = 0;
    }

    S[0] = 0;
    Z[0] = 0;

    /* The specification does not provide the exact recurrence for the
       "final accumulated value associated with xor state v".
       Narrowest faithful interpretation: maintain per-state accumulator
       as the count of occurrences of that prefix-xor value, modulo P,
       using the "most recent index" consistently (tracked but not otherwise
       constrained by the spec). */
    last[0] = 0;
    acc[0] = 1; /* S[0] = 0 is an initial occurrence. */

    for (int i = 1; i <= N; i++) {
        int x;
        if (scanf("%d", &x) != 1) x = 0; /* malformed input => undefined; keep deterministic */
        S[i] = S[i - 1] ^ x;

        Z[i] = Z[i - 1] + (S[i] == 0 ? 1 : 0);

        /* Update per-state accumulator for current prefix xor. */
        uint32_t v = (uint32_t)S[i] & (MAXV - 1u);
        (void)last[v]; /* tracked per invariant; not otherwise used by this minimal interpretation */
        last[v] = i;
        acc[v] = mod_add(acc[v], 1);
    }

    int final_xor = S[N];
    long long ans = 0;

    if (final_xor != 0) {
        ans = acc[(uint32_t)final_xor & (MAXV - 1u)] % MOD;
    } else {
        /* Spec: assumes Z(N) >= 1; otherwise behavior undefined. */
        long long pow_term = mod_pow(2, (long long)Z[N] - 1);
        long long sum_nonzero = 0;
        for (uint32_t v = 1; v < MAXV; v++) {
            sum_nonzero = mod_add(sum_nonzero, acc[v]);
        }
        ans = mod_add(pow_term, sum_nonzero);
    }

    if (ans < 0) ans %= MOD;
    if (ans < 0) ans += MOD;
    if (ans >= MOD) ans %= MOD;

    printf("%lld\n", ans);

    free(S);
    free(Z);
    free(last);
    free(acc);
    return 0;
}

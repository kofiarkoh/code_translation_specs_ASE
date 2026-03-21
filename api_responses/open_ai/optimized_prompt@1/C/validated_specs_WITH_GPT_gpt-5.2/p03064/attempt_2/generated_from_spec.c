#include <stdio.h>
#include <stdint.h>

#define MOD 998244353u

/*
  Fixed internal bound for sum-indexed DP.
  The specification states the program assumes the total sum S fits within
  a fixed internal bound and performs no validation; exceeding it is undefined.
*/
#ifndef MAXS
#define MAXS 200000
#endif

static uint32_t D[MAXS + 1];
static uint32_t E[MAXS + 1];

static inline uint32_t addmod(uint32_t a, uint32_t b) {
    uint32_t s = a + b;
    if (s >= MOD) s -= MOD;
    return s;
}

static inline uint32_t mulmod_u32(uint32_t a, uint32_t b) {
    return (uint32_t)((uint64_t)a * (uint64_t)b % (uint64_t)MOD);
}

int main(void) {
    long long first;
    if (scanf("%lld", &first) != 1) {
        /* Behavior unspecified by spec if first integer cannot be read. */
        return 0;
    }

    /* Initialization does not depend on the first integer's value. */
    for (int i = 0; i <= MAXS; i++) {
        D[i] = 0;
        E[i] = 0;
    }
    D[0] = 1;
    E[0] = 1;

    uint32_t R = 1; /* baseline, becomes 3^n mod MOD */
    int s = 0;      /* running sum of subsequent integers */

    long long x;
    while (scanf("%lld", &x) == 1) {
        /* Spec assumes nonnegative and within capacity; no validation. */
        int ai = (int)x;

        /* Update DP in decreasing order so each ai is used at most once. */
        for (int t = s; t >= 0; --t) {
            int idx = t + ai; /* may go out of bounds if assumptions violated */
            uint32_t dt = D[t];
            uint32_t et = E[t];

            /* D[idx] += 2 * D[t] (mod MOD) */
            uint32_t addD = dt;
            addD = addmod(addD, dt); /* multiply by 2 */
            D[idx] = addmod(D[idx], addD);

            /* E[idx] += E[t] (mod MOD) */
            E[idx] = addmod(E[idx], et);
        }

        s += ai;
        R = mulmod_u32(R, 3u);
    }

    /* Finalization: start from baseline R and subtract terms. */
    int half = s / 2;
    int64_t ans = (int64_t)R;

    for (int t = 0; t <= half; ++t) {
        ans -= (int64_t)mulmod_u32(3u, D[t]);
    }
    if ((s % 2) == 0) {
        ans -= (int64_t)mulmod_u32(3u, E[half]);
    }

    /* Normalize into [0, MOD-1]. */
    int64_t m = (int64_t)MOD;
    ans %= m;
    if (ans < 0) ans += m;

    printf("%lld\n", (long long)ans);
    return 0;
}

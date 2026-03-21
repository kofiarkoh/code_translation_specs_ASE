#include <stdio.h>
#include <stdint.h>
#include <limits.h>

static inline int clamp_0_255(int x) {
    if (x < 0) return 0;
    if (x > 255) return 255;
    return x;
}

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n == 0) break;

        int m;
        if (scanf("%d", &m) != 1) return 0;

        int inc[16];
        for (int j = 0; j < m; j++) {
            if (scanf("%d", &inc[j]) != 1) return 0;
        }

        /* DP over states 0..255, keeping minimum cost to reach each state after i steps.
           Use 32-bit signed costs as per specification assumption. */
        int32_t dp_prev[256];
        int32_t dp_cur[256];

        /* Initialize: before any targets, only state 128 is reachable with cost 0. */
        for (int s = 0; s < 256; s++) dp_prev[s] = INT32_MAX;
        dp_prev[128] = 0;

        for (int i = 0; i < n; i++) {
            int target;
            if (scanf("%d", &target) != 1) return 0;

            for (int s = 0; s < 256; s++) dp_cur[s] = INT32_MAX;

            for (int s = 0; s < 256; s++) {
                int32_t base = dp_prev[s];
                if (base == INT32_MAX) continue;

                for (int j = 0; j < m; j++) {
                    int ns = clamp_0_255(s + inc[j]);
                    int diff = target - ns;
                    /* Cost is (target - state)^2; computed in 64-bit to avoid intermediate overflow,
                       then narrowed to int32_t. If it overflows int32_t, behavior is unspecified
                       per the specification's contract. */
                    int64_t add = (int64_t)diff * (int64_t)diff;
                    int64_t cand64 = (int64_t)base + add;
                    int32_t cand = (int32_t)cand64;

                    if (cand < dp_cur[ns]) dp_cur[ns] = cand;
                }
            }

            for (int s = 0; s < 256; s++) dp_prev[s] = dp_cur[s];
        }

        int32_t ans = INT32_MAX;
        for (int s = 0; s < 256; s++) {
            if (dp_prev[s] < ans) ans = dp_prev[s];
        }

        /* Exactly one line/value per instance. */
        printf("%d\n", (int)ans);
    }

    return 0;
}

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 998244353LL

int main(void) {
    int n;
    int s;
    if (scanf("%d %d", &n, &s) != 2) return 0;

    int *A = (int *)malloc((size_t)n * sizeof(int));
    if (!A) return 0;
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &A[i]) != 1) {
            free(A);
            return 0;
        }
    }

    /* dp0 and dp1 are defined for j in [0, s]. We store only the previous row
       for dp0 and dp1, since transitions depend only on i-1. */
    int64_t *dp0_prev = (int64_t *)calloc((size_t)(s + 1), sizeof(int64_t));
    int64_t *dp0_cur  = (int64_t *)calloc((size_t)(s + 1), sizeof(int64_t));
    int64_t *dp1_prev = (int64_t *)calloc((size_t)(s + 1), sizeof(int64_t));
    int64_t *dp1_cur  = (int64_t *)calloc((size_t)(s + 1), sizeof(int64_t));
    if (!dp0_prev || !dp0_cur || !dp1_prev || !dp1_cur) {
        free(A);
        free(dp0_prev); free(dp0_cur); free(dp1_prev); free(dp1_cur);
        return 0;
    }

    /* Initialization at i = 0 */
    dp0_prev[0] = 1;
    dp1_prev[0] = 1;
    if (A[0] <= s) {
        dp0_prev[A[0]] = (dp0_prev[A[0]] + 1) % MOD;
        dp1_prev[A[0]] = (dp1_prev[A[0]] + 1) % MOD;
    }

    for (int i = 1; i < n; i++) {
        /* Clear current dp0 row */
        for (int j = 0; j <= s; j++) dp0_cur[j] = 0;

        /* Carry forward and optionally append A[i] */
        int ai = A[i];
        for (int j = 0; j <= s; j++) {
            int64_t v = dp0_prev[j] % MOD;
            if (v) {
                dp0_cur[j] += v;
                if (dp0_cur[j] >= MOD) dp0_cur[j] -= MOD;

                /* Preconditions assume A[i] is non-negative; otherwise indexing may be invalid. */
                if (j + ai <= s) {
                    int idx = j + ai;
                    dp0_cur[idx] += v;
                    dp0_cur[idx] %= MOD;
                }
            }
        }

        /* Inject base contributions */
        dp0_cur[0] += 1;
        if (dp0_cur[0] >= MOD) dp0_cur[0] -= MOD;
        if (ai <= s) {
            dp0_cur[ai] += 1;
            dp0_cur[ai] %= MOD;
        }

        /* Accumulate dp1 across indices: dp1(i,j) = dp1(i-1,j) + dp0(i,j) */
        for (int j = 0; j <= s; j++) {
            int64_t x = dp1_prev[j] + dp0_cur[j];
            x %= MOD;
            dp1_cur[j] = x;
        }

        /* Swap prev/cur buffers */
        int64_t *tmp;
        tmp = dp0_prev; dp0_prev = dp0_cur; dp0_cur = tmp;
        tmp = dp1_prev; dp1_prev = dp1_cur; dp1_cur = tmp;
    }

    /* Output dp1(n-1, s) */
    printf("%lld\n", (long long)(dp1_prev[s] % MOD));

    free(A);
    free(dp0_prev);
    free(dp0_cur);
    free(dp1_prev);
    free(dp1_cur);
    return 0;
}

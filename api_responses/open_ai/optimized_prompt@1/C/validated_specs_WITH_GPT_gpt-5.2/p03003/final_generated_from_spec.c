#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 1000000007LL

static inline int64_t mod_norm(int64_t x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

int main(void) {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) {
        return 0; /* unspecified behavior on malformed input; exit quietly */
    }

    int *a = (int *)malloc((size_t)n * sizeof(int));
    int *b = (int *)malloc((size_t)m * sizeof(int));
    if (!a || !b) {
        /* unspecified by spec; but avoid deref NULL */
        return 0;
    }

    for (int i = 0; i < n; i++) {
        if (scanf("%d", &a[i]) != 1) return 0;
    }
    for (int j = 0; j < m; j++) {
        if (scanf("%d", &b[j]) != 1) return 0;
    }

    /*
      dp[i][j] = number of distinct common subsequences between
                 a[0..i-1] and b[0..j-1], including empty subsequence, mod MOD.

      Base: dp[0][j] = dp[i][0] = 1.

      Recurrence for distinct common subsequences:
        if a[i-1] != b[j-1]:
            dp[i][j] = dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1]
        else:
            dp[i][j] = dp[i-1][j] + dp[i][j-1]
      All modulo MOD with normalization to [0, MOD-1].

      This matches the invariant described and runs in O(n*m) time and space.
    */

    size_t rows = (size_t)(n + 1);
    size_t cols = (size_t)(m + 1);
    int64_t *dp = (int64_t *)malloc(rows * cols * sizeof(int64_t));
    if (!dp) {
        return 0;
    }

    for (int i = 0; i <= n; i++) {
        dp[(size_t)i * cols + 0] = 1;
    }
    for (int j = 0; j <= m; j++) {
        dp[0 * cols + (size_t)j] = 1;
    }

    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= m; j++) {
            int64_t up = dp[(size_t)(i - 1) * cols + (size_t)j];
            int64_t left = dp[(size_t)i * cols + (size_t)(j - 1)];
            int64_t diag = dp[(size_t)(i - 1) * cols + (size_t)(j - 1)];
            int64_t val;
            if (a[i - 1] != b[j - 1]) {
                val = up + left - diag;
            } else {
                val = up + left;
            }
            dp[(size_t)i * cols + (size_t)j] = mod_norm(val);
        }
    }

    int64_t ans = dp[(size_t)n * cols + (size_t)m];
    printf("%lld\n", (long long)ans);

    free(dp);
    free(a);
    free(b);
    return 0;
}

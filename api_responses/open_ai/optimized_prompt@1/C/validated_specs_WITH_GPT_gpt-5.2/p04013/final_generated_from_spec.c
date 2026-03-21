#include <stdio.h>

int main(void) {
    int n, a;
    if (scanf("%d %d", &n, &a) != 2) return 0;

    int x[n];
    int sum = 0;
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &x[i]) != 1) return 0;
        sum += x[i];
    }

    /* Undefined behavior is acceptable per spec if sum < 0, a == 0, etc. */

    /* dp[i][k][s] = number of ways to choose exactly k elements from first i elements with sum s */
    int dp[n + 1][n + 1][sum + 1];

    for (int i = 0; i <= n; i++) {
        for (int k = 0; k <= n; k++) {
            for (int s = 0; s <= sum; s++) {
                dp[i][k][s] = 0;
            }
        }
    }
    dp[0][0][0] = 1;

    for (int i = 0; i < n; i++) {
        for (int k = 0; k <= i; k++) {
            for (int s = 0; s <= sum; s++) {
                int cur = dp[i][k][s];
                if (cur == 0) continue;

                /* Not take x[i] */
                dp[i + 1][k][s] += cur;

                /* Take x[i] if sum stays within non-negative representable range */
                if (s + x[i] <= sum) {
                    dp[i + 1][k + 1][s + x[i]] += cur;
                }
            }
        }
    }

    int ans = 0;
    for (int k = 1; k <= n; k++) {
        int target_sum = k * a;
        if (0 <= target_sum && target_sum <= sum) {
            ans += dp[n][k][target_sum];
        }
    }

    printf("%d\n", ans);
    return 0;
}

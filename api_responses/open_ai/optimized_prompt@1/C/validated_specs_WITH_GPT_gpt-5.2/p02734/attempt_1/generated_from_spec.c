#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 998244353u

int main(void) {
    int n;
    int s;
    if (scanf("%d %d", &n, &s) != 2) return 0;

    int *a = (int *)malloc((size_t)n * sizeof(int));
    if (!a) return 0;
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &a[i]) != 1) {
            free(a);
            return 0;
        }
    }

    /*
      dp[j] = number of ways to choose a (possibly empty) subsequence from the
      processed prefix such that its total sum is exactly j, modulo MOD.
      Only j in [0..s] are stored; sums > s are ignored.
    */
    uint32_t *dp = (uint32_t *)calloc((size_t)(s + 1), sizeof(uint32_t));
    if (!dp) {
        free(a);
        return 0;
    }
    dp[0] = 1u; /* empty subsequence */

    uint32_t ans = 0u;

    for (int i = 0; i < n; i++) {
        int x = a[i];

        /*
          For each element, we:
          1) Add dp[s] to answer: for every subsequence formed from previous elements
             with sum s, we can append current element (or not) later; choosing cut
             position at the end of that prefix contributes once per such subsequence.
             This matches counting (T,k) where the cut is before considering current element.
          2) Update dp to account for optionally including current element in the subsequence.
             Standard 0/1 subsequence-sum DP: dp'[j] = dp[j] + dp[j-x] (if j>=x).
        */

        ans += dp[s];
        if (ans >= MOD) ans -= MOD;

        if (x <= s) {
            for (int j = s; j >= x; j--) {
                uint32_t v = dp[j] + dp[j - x];
                if (v >= MOD) v -= MOD;
                dp[j] = v;
            }
        }
        /* if x > s, including it would exceed stored range; ignored per specification */
    }

    /*
      Finally, add dp[s] once more to account for cut position after the last element
      of the chosen subsequence (k = |T|). This also includes the empty subsequence,
      but the overall counting is over non-empty subsequences; the recurrence above
      counts pairs in a way that the empty subsequence contributes exactly once here
      when s==0. To enforce "non-empty subsequence" precisely, subtract that case.
    */
    ans += dp[s];
    ans %= MOD;

    /* Remove contribution from empty subsequence when s==0: it has exactly one cut (k=0). */
    if (s == 0) {
        /* dp[0] includes empty subsequence; it contributes exactly 1 pair (T empty, k=0). */
        ans = (ans + MOD - 1u) % MOD;
    }

    printf("%u\n", ans);

    free(dp);
    free(a);
    return 0;
}

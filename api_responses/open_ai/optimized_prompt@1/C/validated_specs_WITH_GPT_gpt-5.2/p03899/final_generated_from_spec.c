#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <limits.h>

int main(void) {
    int n, m, k;
    if (scanf("%d %d %d", &n, &m, &k) != 3) return 0;

    int *A = (int *)malloc((size_t)n * sizeof(int));
    if (!A) return 0;
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &A[i]) != 1) {
            free(A);
            return 0;
        }
    }

    /* DP invariant:
       dp_prev[j] = max sum selecting exactly (t-1) elements with (t-1)-th at j
       dp_cur[j]  = max sum selecting exactly t elements with t-th at j
       Transition for t>=2:
         dp_cur[j] = t*A[j] + max_{i in [j-(m+1), j-1]} dp_prev[i]
       For t=1:
         dp_cur[j] = 1*A[j] for all j
       Answer: max_j dp_k[j]
    */

    int64_t *dp_prev = (int64_t *)malloc((size_t)n * sizeof(int64_t));
    int64_t *dp_cur  = (int64_t *)malloc((size_t)n * sizeof(int64_t));
    if (!dp_prev || !dp_cur) {
        free(A);
        free(dp_prev);
        free(dp_cur);
        return 0;
    }

    /* Initialize for t = 1 */
    for (int j = 0; j < n; j++) dp_cur[j] = (int64_t)A[j];

    /* For t from 2..k, compute next layer using a sliding window maximum */
    for (int t = 2; t <= k; t++) {
        /* swap dp_prev and dp_cur */
        int64_t *tmp = dp_prev;
        dp_prev = dp_cur;
        dp_cur = tmp;

        /* Monotonic deque of indices for dp_prev, maintaining decreasing dp_prev values.
           Window for j is [j-(m+1), j-1]. */
        int *dq = (int *)malloc((size_t)n * sizeof(int));
        if (!dq) {
            free(A);
            free(dp_prev);
            free(dp_cur);
            return 0;
        }
        int head = 0, tail = 0;

        for (int j = 0; j < n; j++) {
            int add_idx = j - 1;
            if (add_idx >= 0) {
                while (tail > head && dp_prev[dq[tail - 1]] <= dp_prev[add_idx]) tail--;
                dq[tail++] = add_idx;
            }

            int min_idx = j - (m + 1);
            while (tail > head && dq[head] < min_idx) head++;

            if (tail == head) {
                /* No valid previous index; value not required to be meaningful per spec.
                   Use a very negative sentinel to avoid accidental selection. */
                dp_cur[j] = (int64_t)LLONG_MIN / 4;
            } else {
                int best_i = dq[head];
                dp_cur[j] = dp_prev[best_i] + (int64_t)t * (int64_t)A[j];
            }
        }

        free(dq);
    }

    /* If k == 1, dp_cur already holds layer 1; otherwise dp_cur holds layer k after loop. */
    int64_t ans = dp_cur[0];
    for (int j = 1; j < n; j++) {
        if (dp_cur[j] > ans) ans = dp_cur[j];
    }

    printf("%lld\n", (long long)ans);

    free(A);
    free(dp_prev);
    free(dp_cur);
    return 0;
}

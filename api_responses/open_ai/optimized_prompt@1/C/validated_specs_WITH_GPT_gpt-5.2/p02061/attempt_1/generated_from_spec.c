#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MAXN 114513

int main(void) {
    int Q;
    if (scanf("%d", &Q) != 1) {
        return 1; /* undefined output per spec */
    }
    if (Q < 0) {
        /* Spec only states Q >= 0 as precondition; keep narrow behavior:
           treat negative as read failure-like termination. */
        return 1;
    }

    int *queries = NULL;
    if (Q > 0) {
        queries = (int *)malloc((size_t)Q * sizeof(int));
        if (!queries) return 1;
    }

    int maxn = 0;
    for (int i = 0; i < Q; i++) {
        int n;
        if (scanf("%d", &n) != 1) {
            free(queries);
            return 1; /* undefined output per spec */
        }
        queries[i] = n;
        if (n > maxn) maxn = n;
    }

    if (maxn > MAXN) {
        /* Behavior only defined up to MAXN; keep narrow behavior:
           terminate with nonzero and no defined output. */
        free(queries);
        return 1;
    }

    /* Precompute cumulative counts up to maxn. */
    int *cum = NULL;
    if (maxn >= 0) {
        cum = (int *)calloc((size_t)(maxn + 1), sizeof(int));
        if (!cum) {
            free(queries);
            return 1;
        }
    }

    /* For each k, decide if product of proper divisors >= 2k.
       Use capped multiplication: once product >= 2k, we can stop and treat as >= 2k. */
    for (int k = 2; k <= maxn; k++) {
        int prev = cum[k - 1];
        int64_t cap = (int64_t)2 * (int64_t)k;
        int64_t prod = 1;

        /* Multiply all positive divisors d of k with d < k. */
        for (int d = 1; d * (int64_t)d <= k; d++) {
            if (k % d != 0) continue;

            int q = k / d;

            /* d is a divisor; include if proper (d < k). */
            if (d < k) {
                if (prod < cap) {
                    /* cap-aware multiply to avoid overflow and preserve comparison semantics */
                    if (d != 0 && prod > (cap - 1) / d) prod = cap;
                    else {
                        prod *= d;
                        if (prod >= cap) prod = cap;
                    }
                }
            }

            /* q is the paired divisor; include if distinct and proper (q < k). */
            if (q != d && q < k) {
                if (prod < cap) {
                    if (q != 0 && prod > (cap - 1) / q) prod = cap;
                    else {
                        prod *= q;
                        if (prod >= cap) prod = cap;
                    }
                }
            }

            if (prod >= cap) break; /* capped above by 2k for decision purposes */
        }

        cum[k] = prev + (prod >= cap ? 1 : 0);
    }

    for (int i = 0; i < Q; i++) {
        int n = queries[i];
        if (n < 2) {
            puts("0");
        } else {
            /* n is within [0, maxn] by construction */
            printf("%d\n", cum[n]);
        }
    }

    free(cum);
    free(queries);
    return 0;
}

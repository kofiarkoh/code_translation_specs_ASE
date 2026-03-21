#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* No explicit error handling per specification */
        return 0;
    }

    /* Fixed memory proportional to n (expected n <= 100000) */
    int *tails = (int *)malloc((size_t)n * sizeof(int));
    if (!tails) {
        /* No explicit error handling per specification */
        return 0;
    }

    int len = 0;
    for (int i = 0; i < n; i++) {
        int x;
        if (scanf("%d", &x) != 1) {
            /* No explicit error handling per specification */
            free(tails);
            return 0;
        }

        /* Find the first position p such that tails[p] >= x (lower_bound).
           This maintains tails in non-decreasing order and enforces strictly
           increasing subsequence length (duplicates do not extend length). */
        int lo = 0, hi = len;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (tails[mid] < x) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        tails[lo] = x;
        if (lo == len) {
            len++;
        }
    }

    printf("%d", len);

    free(tails);
    return 0;
}

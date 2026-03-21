#include <stdio.h>

#define MAXC 106

int main(void) {
    int rows, cols;
    if (scanf("%d %d", &rows, &cols) != 2) {
        return 0; /* Undefined by spec on malformed/missing input; exit quietly */
    }

    /* Fixed-size storage for one row, bounded independent of number of rows */
    int a[MAXC];

    long long total = 0; /* nonnegative, monotonically nondecreasing */

    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < cols; c++) {
            int v;
            if (scanf("%d", &v) != 1) {
                return 0; /* Undefined by spec; exit quietly */
            }
            /* For rows after the first, treat values as complemented (0<->1). */
            if (r > 0) v = 1 - v;
            a[c] = v;
        }

        /* Process row by shrinking a contiguous active segment [l, rr] to empty. */
        int l = 0;
        int rr = cols - 1;
        while (l <= rr) {
            /* Examine boundary elements; if 1, count a "removal" and remove it. */
            if (a[l] == 1) {
                total++;
                l++;
                continue;
            }
            if (a[rr] == 1) {
                total++;
                rr--;
                continue;
            }

            /* If neither boundary is 1, deterministically remove one boundary.
               The spec does not define any other transformation; choose the
               narrowest interpretation consistent with "shrinks until all
               positions are processed": remove the leftmost element. */
            l++;
        }
    }

    printf("%lld\n", total);
    return 0;
}

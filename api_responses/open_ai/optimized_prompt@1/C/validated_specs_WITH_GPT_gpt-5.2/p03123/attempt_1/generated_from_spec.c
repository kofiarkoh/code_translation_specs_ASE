#include <stdio.h>
#include <stdint.h>

static int feasible(int64_t R, int64_t B, int64_t k) {
    /* Feasibility predicate implied by the arithmetic constraints:
       There must exist an integer x in [0, k] such that:
         2*x + (k - x) <= R   and   x + 2*(k - x) <= B
       i.e.:
         x <= R - k
         x >= 2*k - B
       plus bounds 0 <= x <= k.
       This is monotone in k for non-negative inputs, matching the spec. */
    int64_t lo = 0;
    int64_t hi = k;

    int64_t lo2 = 2 * k - B;
    if (lo2 > lo) lo = lo2;

    int64_t hi2 = R - k;
    if (hi2 < hi) hi = hi2;

    return lo <= hi;
}

int main(void) {
    int64_t R, B;
    if (scanf("%" SCNd64 " %" SCNd64, &R, &B) != 2) {
        /* Undefined behavior per specification if input cannot be read;
           we simply return without output. */
        return 0;
    }

    /* Fixed search bound implied by the spec: output <= 2,222,221,
       so k (which is output+1) is searched up to 2,222,222. */
    int64_t low = 0;
    int64_t high = 2222222; /* inclusive upper bound for k */

    while (low < high) {
        int64_t mid = low + (high - low + 1) / 2;
        if (feasible(R, B, mid)) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    /* Output is one less than the maximum feasible k. */
    printf("%" PRId64 "\n", low - 1);
    return 0;
}

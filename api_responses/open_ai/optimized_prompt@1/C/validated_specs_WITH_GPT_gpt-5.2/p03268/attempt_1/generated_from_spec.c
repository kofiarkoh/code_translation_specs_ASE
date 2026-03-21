#include <stdio.h>
#include <stdint.h>

int main(void) {
    int64_t N, K;
    if (scanf("%" SCNd64 " %" SCNd64, &N, &K) != 2) {
        /* Undefined behavior per specification if input is missing/invalid. */
        return 0;
    }

    /* Preconditions: K != 0, N >= 0. No explicit error handling. */

    int64_t q = N / K; /* C99+ integer division truncates toward zero. With N>=0, matches floor(N/K) for K>0. */

    int64_t ans;
    if ((K & 1) != 0) {
        ans = q * q * q;
    } else {
        int64_t q2 = (N + (K / 2)) / K;
        ans = q * q * q + q2 * q2 * q2;
    }

    printf("%" PRId64 "\n", ans);
    return 0;
}

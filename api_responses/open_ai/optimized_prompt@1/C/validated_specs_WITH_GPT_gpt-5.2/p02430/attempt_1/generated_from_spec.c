#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

int main(void) {
    int n, k;
    if (scanf("%d %d", &n, &k) != 2) {
        /* Unspecified behavior per spec if input cannot be read; exit quietly. */
        return 0;
    }

    /* If k is outside [0, n], no outputs are produced. */
    if (n < 0 || k < 0 || k > n) {
        return 0;
    }

    /*
     * Use uint64_t for masks. The spec assumes n is small enough that:
     * - enumerating 2^n is feasible
     * - shifting/masking is valid on the target platform
     * We still avoid undefined behavior for n >= 64 by not shifting by >= 64.
     * If n >= 64, behavior is unspecified by the spec anyway; we choose to
     * produce no output rather than invoke undefined behavior.
     */
    if (n >= 64) {
        return 0;
    }

    uint64_t limit = (n == 64) ? UINT64_MAX : (1ULL << (unsigned)n);

    for (uint64_t p = 0; p < limit; ++p) {
        int cnt = 0;
        for (int i = 0; i < n; ++i) {
            if ((p >> (unsigned)i) & 1ULL) {
                ++cnt;
            }
        }
        if (cnt != k) continue;

        /* Output: mask value followed by ascending set bit indices. */
        printf("%" PRIu64, p);
        for (int i = 0; i < n; ++i) {
            if ((p >> (unsigned)i) & 1ULL) {
                printf(" %d", i);
            }
        }
        putchar('\n');
    }

    return 0;
}

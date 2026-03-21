#include <stdio.h>

static int popcount_int(int x) {
    /* Count set bits using unsigned to avoid issues with signed shifts. */
    unsigned int u = (unsigned int)x;
    int c = 0;
    while (u) {
        c += (int)(u & 1u);
        u >>= 1;
    }
    return c;
}

int main(void) {
    int n, k;
    if (scanf("%d %d", &n, &k) != 2) {
        /* Unspecified behavior per spec if input is not two integers. */
        return 0;
    }

    /* Assumption: (1 << n) is well-defined for int and feasible to iterate. */
    int limit = (1 << n);

    for (int p = 0; p < limit; ++p) {
        if (popcount_int(p) == k) {
            printf("%d:", p);
            for (int i = 0; i < n; ++i) {
                if (p & (1 << i)) {
                    printf(" %d", i);
                }
            }
            putchar('\n');
        }
    }

    return 0;
}

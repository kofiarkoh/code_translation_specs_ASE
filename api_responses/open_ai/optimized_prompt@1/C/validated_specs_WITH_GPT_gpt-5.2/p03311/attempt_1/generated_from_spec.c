#include <stdio.h>
#include <stdlib.h>

static int cmp_long_long(const void *a, const void *b) {
    const long long x = *(const long long *)a;
    const long long y = *(const long long *)b;
    if (x < y) return -1;
    if (x > y) return 1;
    return 0;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Undefined behavior per specification if input is missing/invalid. */
        return 0;
    }

    /* Specification assumes n >= 1; behavior is undefined otherwise. */
    long long b[n]; /* stack allocation as required */

    for (int i = 0; i < n; i++) {
        long long x;
        if (scanf("%lld", &x) != 1) {
            /* Undefined behavior per specification if required inputs are missing/invalid. */
            return 0;
        }
        /* b[i] = x[i] - (i+1) with 1-based index */
        b[i] = x - (long long)(i + 1);
    }

    qsort(b, (size_t)n, sizeof(b[0]), cmp_long_long);

    /* Any median minimizes sum of absolute deviations; choose lower median. */
    long long median = b[n / 2];

    long long sum = 0;
    for (int i = 0; i < n; i++) {
        long long d = b[i] - median;
        if (d < 0) d = -d;
        sum += d;
    }

    printf("%lld\n", sum);
    return 0;
}

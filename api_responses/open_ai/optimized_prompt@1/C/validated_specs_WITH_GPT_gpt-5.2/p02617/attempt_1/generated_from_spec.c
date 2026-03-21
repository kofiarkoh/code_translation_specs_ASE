#include <stdio.h>
#include <stdint.h>

int main(void) {
    long long N;
    if (scanf("%lld", &N) != 1) {
        return 0; /* Undefined by spec on malformed input; exit quietly. */
    }

    /* Baseline:
       B = sum_{k=1..N} k(k+1)/2
       Compute in 64-bit signed as specified (no overflow checks). */
    long long B = 0;
    for (long long k = 1; k <= N; k++) {
        B += (k * (k + 1)) / 2;
    }

    long long ans = B;

    /* Consume exactly N-1 pairs. */
    for (long long i = 1; i <= N - 1; i++) {
        long long a, b;
        if (scanf("%lld %lld", &a, &b) != 2) {
            return 0; /* Undefined by spec on insufficient input. */
        }

        long long x = (a < b) ? a : b;
        long long y = (a < b) ? b : a;

        /* Reduction contribution: x * (N - y + 1) */
        ans -= x * (N - y + 1);
    }

    printf("%lld\n", ans);
    return 0;
}

#include <stdio.h>

int main(void) {
    long long N, K;
    if (scanf("%lld %lld", &N, &K) != 2) {
        /* Unspecified by the specification; no explicit error handling. */
        return 0;
    }

    /* Preconditions assumed by specification: K != 0, N >= 0, no overflow. */
    long long x = N / K;
    long long result;

    if (K % 2 != 0) {
        result = x * x * x;
    } else {
        long long y = (N + K / 2) / K;
        result = x * x * x + y * y * y;
    }

    printf("%lld\n", result);
    return 0;
}

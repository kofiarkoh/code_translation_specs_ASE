#include <stdio.h>

int main(void) {
    int N, K;
    if (scanf("%d%d", &N, &K) != 2) {
        /* Unspecified behavior per specification; return without output. */
        return 0;
    }

    /* Preconditions: N > 0, no int overflow, sufficient stack space. */
    int denom[N];

    int multi = 1;

    /* Construct denominators for i = N down to 1, storing at denom[i-1]. */
    for (int i = N; i >= 1; --i) {
        while (K > multi * i) {
            multi *= 2;
        }
        denom[i - 1] = multi * N;
    }

    /* denom_1 corresponds to i = 1, stored at denom[0]. */
    int denom1 = denom[0];

    long double S = 0.0L;
    for (int i = 0; i < N; ++i) {
        S += (long double)denom1 / (long double)denom[i];
    }

    long double result = S / (long double)denom1;

    /* Fixed-point with exactly 12 digits after decimal, no trailing newline. */
    printf("%.12Lf", result);

    return 0;
}

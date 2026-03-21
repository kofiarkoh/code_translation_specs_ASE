#include <stdio.h>

int main(void) {
    long long A, B, X;
    if (scanf("%lld %lld %lld", &A, &B, &X) != 3) {
        return 0; /* Unspecified behavior on malformed input; exit quietly. */
    }

    long long result;

    if (A >= 2 * B) {
        /* Use only 500-unit blocks: result = B * ceil(X / 500). */
        long long n = (X + 500 - 1) / 500;
        result = B * n;
    } else {
        /* Mixed scheme based on 1000-unit blocks with remainder handling. */
        long long q = X / 1000;
        long long r = X % 1000;

        result = q * A;

        if (r == 0) {
            /* add nothing */
        } else if (r <= 500 && B < A) {
            result += B;
        } else {
            result += A;
        }
    }

    printf("%lld\n", result);
    return 0;
}

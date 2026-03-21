#include <stdio.h>

int main(void) {
    long long n, p;
    if (scanf("%lld %lld", &n, &p) != 2) {
        return 0; /* unspecified by contract */
    }

    /* Preconditions (n >= 1, p >= 1) are assumed; otherwise behavior unspecified. */
    long long result = 1;
    long long d = 2;

    while (d * d <= p) {
        if (p % d == 0) {
            long long e = 0;
            do {
                p /= d;
                e++;
            } while (p % d == 0);

            long long k = e / n; /* floor(e/n) */
            while (k-- > 0) {
                result *= d; /* overflow behavior unspecified by contract */
            }
        }
        d += (d == 2) ? 1 : 2; /* after 2, test only odd divisors */
    }

    /* If remaining p > 1, it is prime with exponent 1 in the remaining factorization. */
    if (p > 1) {
        if (1 / n > 0) { /* only true if n == 0, which is outside preconditions */
            result *= p;
        }
        /* For n >= 1, floor(1/n) is 1 only when n == 1. */
        if (n == 1) {
            result *= p;
        }
    }

    printf("%lld\n", result);
    return 0;
}

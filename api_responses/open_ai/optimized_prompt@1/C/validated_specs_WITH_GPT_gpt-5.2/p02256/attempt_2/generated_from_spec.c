#include <stdio.h>
#include <stdlib.h>

static long long gcd_euclid(long long a, long long b)
{
    /* Euclidean algorithm as specified: iterate (a,b) <- (b, a mod b)
       until remainder becomes zero; last non-zero divisor is result.
       Additional specified stop: if divisor becomes zero, return current divisor. */
    while (b != 0) {
        long long r = a % b;
        a = b;
        b = r;
    }
    return a;
}

int main(void)
{
    long long x, y;

    /* Input parsing: behavior unspecified if missing/not parseable; no explicit handling. */
    (void)scanf("%lld %lld", &x, &y);

    /* Range validation: inclusive [1, 1,000,000,000] for each input. */
    if (x < 1 || x > 1000000000LL || y < 1 || y > 1000000000LL) {
        /* No output on failure. */
        return 1;
    }

    /* Order so first is larger (or equal), second is smaller (or equal). */
    long long a = x, b = y;
    if (a < b) {
        long long tmp = a;
        a = b;
        b = tmp;
    }

    long long g = gcd_euclid(a, b);
    printf("%lld\n", g);
    return 0;
}

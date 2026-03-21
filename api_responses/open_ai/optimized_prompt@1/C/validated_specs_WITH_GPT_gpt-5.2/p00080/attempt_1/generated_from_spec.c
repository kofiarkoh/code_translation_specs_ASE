#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <float.h>

int main(void) {
    /* Specification does not define the sentinel value; choose the narrowest
       conventional interpretation: a single designated integer value, -1. */
    const long long SENTINEL = -1;

    /* Relative cube error tolerance */
    const double eps = 1e-5;

    /* Fixed number of digits after the decimal point (not specified); choose 5
       to align with eps = 1e-5 as a narrow, reasonable interpretation. */
    const int out_digits = 5;

    long long q_ll;
    while (scanf("%lld", &q_ll) == 1) {
        if (q_ll == SENTINEL) break;

        /* Preconditions say q > 0; behavior for non-positive is unspecified.
           We still attempt computation to avoid inventing extra error handling. */
        double q = (double)q_ll;

        /* Initial guess: use exp/log for scale; fall back to 1.0 if not usable. */
        double x;
        if (q > 0.0) {
            x = exp(log(q) / 3.0);
            if (!isfinite(x) || x <= 0.0) x = 1.0;
        } else {
            x = 1.0;
        }

        /* Newton-Raphson for cube root: x_{n+1} = (2x + q/x^2)/3
           Stop when |x^3 - q| < eps * q (relative cube error criterion). */
        for (int iter = 0; iter < 1000000; iter++) {
            double x2 = x * x;
            if (x2 == 0.0) {
                x = 1.0;
                continue;
            }

            double x3 = x2 * x;
            double err = fabs(x3 - q);

            if (q > 0.0) {
                if (err < eps * q) break;
            } else {
                /* For unspecified q <= 0, use a minimal termination condition
                   to reduce risk of non-termination without defining behavior. */
                if (err < eps) break;
            }

            double next = (2.0 * x + q / x2) / 3.0;

            /* If iteration becomes non-finite, stop to avoid infinite loops. */
            if (!isfinite(next)) break;

            /* If no progress due to floating-point limits, stop. */
            if (next == x) break;

            x = next;
        }

        printf("%.*f\n", out_digits, x);
    }

    return 0;
}

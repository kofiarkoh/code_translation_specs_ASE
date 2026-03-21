#include <stdio.h>

static long long digit_sum_ll(long long x) {
    long long s = 0;
    while (x > 0) {
        s += x % 10;
        x /= 10;
    }
    return s;
}

int main(void) {
    long long n;
    if (scanf("%lld", &n) != 1) {
        return 0; /* unspecified by spec if input invalid; exit quietly */
    }
    if (n <= 0) {
        return 0; /* n = 0 => no output; n non-negative per preconditions */
    }

    long long cur = 1;

    for (long long i = 0; i < n; i++) {
        printf("%lld\n", cur);

        long long p = cur;

        /* Candidate evaluation order is significant for tie-breaking:
           1) p+1
           2) for scale = 10, 100, ..., 10^15: the largest integer < next multiple
              of that scale above (p+1), i.e., (ceil((p+1)/scale)*scale - 1)
        */
        long long best = p + 1;
        long long best_ds = digit_sum_ll(best);
        double best_ratio = (double)best / (double)best_ds;

        long long x = p + 1;

        long long scale = 10;
        for (int k = 0; k < 15; k++) {
            /* next multiple of scale above x:
               m = ((x + scale - 1) / scale) * scale
               candidate = m - 1
            */
            long long m = ((x + scale - 1) / scale) * scale;
            long long cand = m - 1;

            /* cand is strictly greater than p because cand >= x and x = p+1 */
            long long ds = digit_sum_ll(cand);
            double ratio = (double)cand / (double)ds;

            /* Use strict < so ties keep earliest encountered candidate. */
            if (ratio < best_ratio) {
                best_ratio = ratio;
                best = cand;
                best_ds = ds;
            }

            scale *= 10;
        }

        (void)best_ds; /* kept to mirror computation; not otherwise needed */
        cur = best;
    }

    return 0;
}

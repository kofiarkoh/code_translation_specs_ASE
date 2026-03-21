#include <stdio.h>

static void print_hms_from_seconds(long long total_seconds) {
    /* Use C's / and % semantics (truncate toward zero; remainder has sign of dividend)
       to preserve behavior even for negative values, as required by the spec. */
    long long h = total_seconds / 3600;
    long long rem = total_seconds % 3600;
    long long m = rem / 60;
    long long s = rem % 60;
    printf("%lld %lld %lld\n", h, m, s);
}

int main(void) {
    long long h, m, s;

    while (1) {
        int rc = scanf("%lld %lld %lld", &h, &m, &s);
        if (rc != 3) {
            /* End-of-input or incomplete triple: terminate without further output. */
            break;
        }

        if (h == -1 && m == -1 && s == -1) {
            /* Sentinel: terminate without output for this triple. */
            break;
        }

        long long T = 3600LL * h + 60LL * m + s;
        long long R = 7200LL - T;

        print_hms_from_seconds(R);
        print_hms_from_seconds(3LL * R);
    }

    return 0;
}

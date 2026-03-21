#include <stdio.h>

int main(void) {
    long long t1, t2;
    long long a1, a2;
    long long b1, b2;

    if (scanf("%lld %lld %lld %lld %lld %lld", &t1, &t2, &a1, &a2, &b1, &b2) != 6) {
        return 0; /* No explicit error handling per specification */
    }

    /* Total combined values over both phases */
    long long totalA = t1 * a1 + t2 * a2;
    long long totalB = t1 * b1 + t2 * b2;

    if (totalA == totalB) {
        puts("infinity");
        return 0;
    }

    /*
      Normalization step to satisfy invariant:
      after normalization, A's phase-1 per-unit value <= B's phase-1 per-unit value.
      Swap the entities' per-phase values if needed.
    */
    if (a1 > b1) {
        long long tmp;
        tmp = a1; a1 = b1; b1 = tmp;
        tmp = a2; a2 = b2; b2 = tmp;
        /* totals are not needed after this point */
    }

    /*
      Derived quantities:
      phase-1 advantage (for B over A):  (b1 - a1) * t1
      phase-2 catch-up (for A over B):   (a2 - b2) * t2
    */
    long long adv1 = (b1 - a1) * t1;
    long long catch2 = (a2 - b2) * t2;

    if (adv1 > catch2) {
        puts("0");
        return 0;
    }

    /*
      Final ratio computation:
      denominator is the per-unit difference in phase 2 (A - B), required nonzero
      in the non-infinity path by precondition.
    */
    long long denom = (a2 - b2);

    long long q = adv1 / denom; /* integer quotient */
    long long r = adv1 % denom; /* remainder */

    long long ans = 2 * q;
    if (r != 0) ans += 1; /* odd if not exactly divisible, even otherwise */

    printf("%lld\n", ans);
    return 0;
}

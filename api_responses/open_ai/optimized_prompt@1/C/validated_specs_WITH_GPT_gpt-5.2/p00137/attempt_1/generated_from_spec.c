#include <stdio.h>

int main(void) {
    int T;
    if (scanf("%d", &T) != 1) {
        return 0; /* Unspecified behavior per spec if input missing/invalid */
    }

    for (int case_idx = 1; case_idx <= T; ++case_idx) {
        int x;
        if (scanf("%d", &x) != 1) {
            return 0; /* Unspecified behavior per spec if input missing/invalid */
        }

        printf("Case %d:\n", case_idx);

        for (int i = 0; i < 10; ++i) {
            /*
             * Deterministic transformation:
             * The spec requires each printed value to be in [0, 9999] and to be
             * derived by repeatedly applying the same transformation to the
             * current value. The narrowest consistent interpretation is to
             * update by squaring and taking the last 4 decimal digits.
             *
             * Overflow is unspecified by the spec; we therefore use 'int' and
             * do not add overflow-avoidance beyond the spec's precondition.
             */
            x = (x * x) % 10000;

            /* Ensure output is within 0..9999 even if x was negative. */
            if (x < 0) x += 10000;

            printf("%d\n", x);
        }
    }

    return 0;
}

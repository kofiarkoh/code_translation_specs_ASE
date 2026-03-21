#include <stdio.h>

int main(void) {
    int A, B, C, X;
    if (scanf("%d %d %d %d", &A, &B, &C, &X) != 4) {
        /* Unspecified behavior per spec if input is missing/not integer.
           Return without producing defined output. */
        return 0;
    }

    int count = 0;

    /* If any maximum is negative, no combinations are considered (effectively zero). */
    if (A < 0 || B < 0 || C < 0) {
        printf("%d\n", 0);
        return 0;
    }

    for (int i = 0; i <= A; i++) {
        for (int j = 0; j <= B; j++) {
            for (int k = 0; k <= C; k++) {
                /* Overflow behavior is unspecified by spec; use int arithmetic directly. */
                if (500 * i + 100 * j + 50 * k == X) {
                    count++;
                }
            }
        }
    }

    printf("%d\n", count);
    return 0;
}

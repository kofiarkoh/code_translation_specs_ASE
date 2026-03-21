#include <stdio.h>

int main(void) {
    int a, b, c;
    if (scanf("%d %d %d", &a, &b, &c) != 3) {
        /* Unspecified behavior per specification; exit without defined output. */
        return 0;
    }

    /* Preconditions state a != 0; if a == 0, division by zero is undefined. */
    int q = b / a; /* C integer division truncates toward zero (C99+). */

    int out = (q < c) ? q : c;
    printf("%d\n", out);
    return 0;
}

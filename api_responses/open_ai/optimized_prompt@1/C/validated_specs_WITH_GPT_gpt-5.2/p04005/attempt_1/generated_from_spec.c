#include <stdio.h>

int main(void) {
    int a, b, c;

    /* Behavior is unspecified if fewer than three valid integers are supplied. */
    if (scanf("%d %d %d", &a, &b, &c) != 3) {
        return 0;
    }

    /* If any value is even, output 0. */
    if ((a % 2) == 0 || (b % 2) == 0 || (c % 2) == 0) {
        printf("0");
        return 0;
    }

    /* All are odd: replace one maximum value with 1, then output product. */
    if (a >= b && a >= c) {
        a = 1; /* If ties, choosing any maximum is not externally observable. */
    } else if (b >= a && b >= c) {
        b = 1;
    } else {
        c = 1;
    }

    /* Overflow behavior is unspecified per specification. */
    printf("%d", a * b * c);
    return 0;
}

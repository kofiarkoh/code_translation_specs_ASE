#include <stdio.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Unspecified behavior per spec if input is missing/malformed. */
        return 0;
    }

    int c1 = 0, c2 = 0, c3 = 0, c4 = 0, c5 = 0, c6 = 0;

    for (int i = 0; i < n; i++) {
        double h;
        if (scanf("%lf", &h) != 1) {
            /* Unspecified behavior per spec if input is missing/malformed. */
            return 0;
        }

        if (h < 165.0) {
            c1++;
        } else if (h < 170.0) {
            c2++;
        } else if (h < 175.0) {
            c3++;
        } else if (h < 180.0) {
            c4++;
        } else if (h < 185.0) {
            c5++;
        } else {
            c6++;
        }
    }

    /* Print exactly six lines, labels 1: through 6:, then '*' repeated count times, then newline. */
    printf("1:");
    for (int i = 0; i < c1; i++) putchar('*');
    putchar('\n');

    printf("2:");
    for (int i = 0; i < c2; i++) putchar('*');
    putchar('\n');

    printf("3:");
    for (int i = 0; i < c3; i++) putchar('*');
    putchar('\n');

    printf("4:");
    for (int i = 0; i < c4; i++) putchar('*');
    putchar('\n');

    printf("5:");
    for (int i = 0; i < c5; i++) putchar('*');
    putchar('\n');

    printf("6:");
    for (int i = 0; i < c6; i++) putchar('*');
    putchar('\n');

    return 0;
}

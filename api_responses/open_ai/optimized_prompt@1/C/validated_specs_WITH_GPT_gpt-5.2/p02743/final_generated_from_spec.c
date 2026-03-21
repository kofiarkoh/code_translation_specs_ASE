#include <stdio.h>

int main(void) {
    double a, b, c;
    if (scanf("%lf %lf %lf", &a, &b, &c) != 3) {
        /* Unspecified by contract for missing/invalid input; exit quietly. */
        return 0;
    }

    /* Postcondition:
       Output "Yes" iff:
         (1) c > a + b
         (2) (c - a - b)^2 > 4*(a*b)
       Otherwise "No".
       Note: Any non-finite results/overflows lead to unspecified outcomes per spec. */
    double sum = a + b;
    double diff = c - sum;

    if (c > sum && (diff * diff) > (4.0 * (a * b))) {
        puts("Yes");
    } else {
        puts("No");
    }

    return 0;
}

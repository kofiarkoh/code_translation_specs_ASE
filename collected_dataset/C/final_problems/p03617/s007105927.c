#include <stdio.h>
static double q, h, s, d, n;
int main(void) {
    long sum = 0;
    scanf("%lf%lf%lf%lf%lf", &q, &h, &s, &d, &n);
    if (8 * q < 4 * h)
        h = 2 * q;
    if (4 * h < 2 * s)
        s = 2 * h;
    if (2 * s < 1 * d)
        d = 2 * s;
    while (n >= 2) {
        sum += d;
        n -= 2;
    }
    while (n >= 1) {
        sum += s;
        n -= 1;
    }
    printf("%ld\n", sum);
    return 0;
}

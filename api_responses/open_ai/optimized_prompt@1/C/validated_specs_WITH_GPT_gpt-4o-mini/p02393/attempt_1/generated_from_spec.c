#include <stdio.h>

int main() {
    int a, b, c;

    // Read three integers from standard input
    scanf("%d %d %d", &a, &b, &c);

    // Output the integers in non-decreasing order
    if (a <= b && b <= c) {
        printf("%d %d %d\n", a, b, c);
    } else if (a <= c && c <= b) {
        printf("%d %d %d\n", a, c, b);
    } else if (b <= a && a <= c) {
        printf("%d %d %d\n", b, a, c);
    } else if (b <= c && c <= a) {
        printf("%d %d %d\n", b, c, a);
    } else if (c <= a && a <= b) {
        printf("%d %d %d\n", c, a, b);
    } else {
        printf("%d %d %d\n", c, b, a);
    }

    return 0;
}

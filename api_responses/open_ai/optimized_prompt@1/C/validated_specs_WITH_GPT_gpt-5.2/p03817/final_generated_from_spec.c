#include <stdio.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Unspecified behavior per specification if input cannot be read. */
        return 0;
    }

    int q = n / 11;
    int r = n % 11;

    int ans;
    if (r == 0) {
        ans = 2 * q;
    } else if (r >= 1 && r <= 6) {
        ans = 2 * q + 1;
    } else {
        /* For non-negative n, this corresponds to 7..10.
           For negative n, behavior follows C's / and % rules; this branch
           also covers r < 0 or r > 10 without adding extra handling. */
        ans = 2 * q + 2;
    }

    printf("%d", ans);
    return 0;
}

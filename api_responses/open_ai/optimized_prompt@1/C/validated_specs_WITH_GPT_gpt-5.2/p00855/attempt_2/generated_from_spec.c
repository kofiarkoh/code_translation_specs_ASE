#include <stdio.h>

static int is_prime(int n) {
    int count = 0;
    int i;

    /* Specification-defined trial division:
       count divisors i in [2, ...] while i*i <= n; prime iff count == 0.
       Note: For n <= 1, this function will (per spec) treat it as prime
       because the loop doesn't execute; overall program behavior for such
       inputs is not guaranteed by the specification. */
    for (i = 2; i * i <= n; i++) {
        if (n % i == 0) {
            count++;
        }
    }
    return (count == 0);
}

int main(void) {
    int x;

    while (scanf("%d", &x) == 1) {
        int L, U;

        if (x == 0) {
            break; /* sentinel: no output */
        }

        /* Find L: greatest prime <= x by decrementing from x */
        L = x;
        while (!is_prime(L)) {
            L--;
        }

        /* Find U: least prime >= x by incrementing from x */
        U = x;
        while (!is_prime(U)) {
            U++;
        }

        printf("%d\n", U - L);
    }

    return 0;
}

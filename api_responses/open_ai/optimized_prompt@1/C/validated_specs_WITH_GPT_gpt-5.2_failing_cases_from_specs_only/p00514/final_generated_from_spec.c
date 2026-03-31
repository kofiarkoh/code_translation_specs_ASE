#include <stdio.h>

#define BASE 1000000000LL
#define DIGS 200

static void big_mul_small(long long a[], int len, int x) {
    long long carry = 0;
    for (int i = 0; i < len; i++) {
        long long t = a[i] * (long long)x + carry; /* may overflow if preconditions violated */
        a[i] = t % BASE;
        carry = t / BASE;
    }
    /* Fixed capacity: any remaining carry is silently discarded (undefined by spec if it happens). */
}

static void big_div_small(long long a[], int len, int x) {
    long long rem = 0;
    for (int i = len - 1; i >= 0; i--) {
        long long cur = a[i] + rem * BASE; /* may overflow if preconditions violated */
        a[i] = cur / x;
        rem = cur % x;
    }
    /* Exact divisibility is assumed by the algorithm under preconditions. */
}

static void big_print(const long long a[], int len) {
    int i = len - 1;
    while (i > 0 && a[i] == 0) i--;
    printf("%lld", a[i]);
    for (i = i - 1; i >= 0; i--) {
        printf("%09lld", a[i]);
    }
    putchar('\n');
}

int main(void) {
    int n, m, r;
    scanf("%d%d%d", &n, &m, &r);

    int rp = r - n * m; /* native int arithmetic; overflow is undefined per spec */

    if (rp < 0) {
        puts("0");
        return 0;
    }
    if (rp == 0) {
        puts("1");
        return 0;
    }

    int k = (n - 1 < rp) ? (n - 1) : rp;

    long long a[DIGS];
    for (int i = 0; i < DIGS; i++) a[i] = 0;
    a[0] = 1;

    /* Compute C(n+rp-1, n-1) via product/division with k = min(n-1, rp). */
    for (int i = 0; i < k; i++) {
        int mult = n + rp - 1 - i;
        big_mul_small(a, DIGS, mult);
    }
    for (int i = 1; i <= k; i++) {
        big_div_small(a, DIGS, i);
    }

    big_print(a, DIGS);
    return 0;
}

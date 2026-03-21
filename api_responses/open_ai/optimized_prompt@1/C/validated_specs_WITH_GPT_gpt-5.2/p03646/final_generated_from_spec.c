#include <stdio.h>

int main(void) {
    long long k;
    if (scanf("%lld", &k) != 1) {
        return 0; /* unspecified behavior per spec; exit quietly */
    }

    if (k >= 50) {
        long long n = 50;
        long long rem = k / 50;
        long long t = k % 50; /* for k>=0, in 0..49; for k<0, C remainder rules apply */
        long long x = 49 + rem;

        long long a[50];
        for (long long i = 0; i < 50; i++) a[i] = x;

        /* Apply the specified iterative transformation */
        for (long long i = 0; i < t; i++) {
            a[i] += 50;
            for (long long j = 0; j < 50; j++) {
                if (j != i) a[j] -= 1;
            }
        }

        printf("%lld\n", n);
        for (long long i = 0; i < 50; i++) {
            printf("%lld", a[i]);
            if (i + 1 < 50) printf(" ");
        }
        /* final newline not required */
        return 0;
    }

    if (k >= 2) { /* 2 <= k < 50 */
        long long n = k;
        printf("%lld\n", n);
        for (long long i = 0; i < n; i++) {
            printf("%lld", n);
            if (i + 1 < n) printf(" ");
        }
        return 0;
    }

    if (k == 0) {
        printf("4\n");
        printf("3 3 3 3");
        return 0;
    }

    /* k == 1 */
    printf("3\n");
    printf("1 0 3");
    return 0;
}

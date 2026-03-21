#include <stdio.h>

int main(void) {
    long long h, w, k;
    if (scanf("%lld %lld %lld", &h, &w, &k) != 3) {
        /* No explicit error handling per specification; just exit. */
        return 0;
    }

    for (long long i = 0; i <= h; i++) {
        for (long long j = 0; j <= w; j++) {
            long long val = (w * i) + (h * j) - 2LL * i * j;
            if (val == k) {
                puts("Yes");
                return 0;
            }
        }
    }

    puts("No");
    return 0;
}

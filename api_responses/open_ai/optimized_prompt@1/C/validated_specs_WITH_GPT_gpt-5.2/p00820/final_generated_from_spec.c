#include <stdio.h>

static int count_reps(int remaining, int min_a, int depth_left) {
    int count = 0;
    int a;

    if (depth_left <= 0) return 0;

    for (a = min_a; (long long)a * (long long)a <= (long long)remaining; ++a) {
        int sq = a * a; /* overflow is undefined per specification */
        int rem2 = remaining - sq;

        /* Using exactly one term */
        if (rem2 == 0) {
            count += 1;
            continue;
        }

        /* Using more terms (up to 4 total), maintaining non-decreasing order */
        if (depth_left > 1) {
            count += count_reps(rem2, a, depth_left - 1);
        }
    }

    return count;
}

int main(void) {
    int n;

    while (scanf("%d", &n) == 1) {
        if (n == 0) break;
        if (n > 0) {
            int ans = count_reps(n, 1, 4);
            printf("%d\n", ans);
        } else {
            /* Specification only defines behavior for positive integers; do nothing. */
        }
    }

    return 0;
}

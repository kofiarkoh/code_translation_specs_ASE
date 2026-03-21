#include <stdio.h>
#include <string.h>

#define MOD 10007
#define MAXN 1005

static int req_bit(char c) {
    /* Specification note: if c is not J/O/I, behavior is undefined due to
       use of an uninitialized value in the original-style program.
       We emulate that by not providing a defined default. */
    int b;
    if (c == 'J') b = 1;      /* bit 0 */
    else if (c == 'O') b = 2; /* bit 1 */
    else if (c == 'I') b = 4; /* bit 2 */
    return b;
}

int main(void) {
    int N;
    char in[MAXN];

    if (scanf("%d %1000s", &N, in) != 2) {
        /* No explicit error handling per specification; just exit. */
        return 0;
    }

    /* DP over subset states of {J,O,I} represented as 3-bit masks 0..7. */
    int dp_prev[8] = {0};
    int dp_next[8] = {0};

    /* Initialization: "before day 0" has only subset {J} as prior state. */
    dp_prev[1] = 1;

    for (int d = 0; d < N; d++) {
        for (int s = 0; s < 8; s++) dp_next[s] = 0;

        int need = req_bit(in[d]);

        for (int prev = 0; prev < 8; prev++) {
            int ways = dp_prev[prev];
            if (ways == 0) continue;

            for (int cur = 0; cur < 8; cur++) {
                /* Must include required participant for day d. */
                if ((cur & need) == 0) continue;

                if (d == 0) {
                    /* Day 0 special: must also include J. */
                    if ((cur & 1) == 0) continue;
                    /* Overlap with prior state {J} is implied by including J,
                       but we keep the general overlap check consistent. */
                } else {
                    /* Day-to-day overlap for d > 0. */
                    if ((cur & prev) == 0) continue;
                }

                dp_next[cur] += ways;
                dp_next[cur] %= MOD;
            }
        }

        for (int s = 0; s < 8; s++) dp_prev[s] = dp_next[s];
    }

    int ans = 0;
    for (int s = 0; s < 8; s++) {
        ans += dp_prev[s];
        ans %= MOD;
    }

    printf("%d\n", ans);
    return 0;
}

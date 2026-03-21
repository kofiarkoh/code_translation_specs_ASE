#include <stdio.h>
#include <string.h>

#define MAX_N 100
#define MAX_R 300
#define BAL_MIN (-300)
#define BAL_MAX 300
#define BAL_RANGE (BAL_MAX - BAL_MIN + 1)
#define BAL_OFF (-BAL_MIN)

/* Use a sentinel for unreachable states. Rewards are non-negative, so -1 works. */
#define UNREACHABLE (-1)

static int max_int(int a, int b) { return (a > b) ? a : b; }

int main(void) {
    int n;
    int Rmax;
    int Bmin;

    if (scanf("%d", &n) != 1) return 0;
    if (scanf("%d %d", &Rmax, &Bmin) != 2) return 0;

    /* DP over (used_resource, balance_index). Two layers for item transitions. */
    static int dp_cur[MAX_R + 1][BAL_RANGE];
    static int dp_nxt[MAX_R + 1][BAL_RANGE];

    /* Initialize all states unreachable. Start at used=0, balance=0 with reward=0. */
    for (int r = 0; r <= MAX_R; r++) {
        for (int b = 0; b < BAL_RANGE; b++) {
            dp_cur[r][b] = UNREACHABLE;
        }
    }
    dp_cur[0][0 + BAL_OFF] = 0;

    for (int i = 0; i < n; i++) {
        int cons, dec, inc, rew;
        if (scanf("%d %d %d %d", &cons, &dec, &inc, &rew) != 4) return 0;

        /* Next layer starts as unreachable. */
        for (int r = 0; r <= MAX_R; r++) {
            for (int b = 0; b < BAL_RANGE; b++) {
                dp_nxt[r][b] = UNREACHABLE;
            }
        }

        /* Transition from all reachable states. */
        for (int r = 0; r <= Rmax; r++) {
            for (int b = 0; b < BAL_RANGE; b++) {
                int cur = dp_cur[r][b];
                if (cur == UNREACHABLE) continue;

                /* no-op */
                dp_nxt[r][b] = max_int(dp_nxt[r][b], cur);

                /* consume: r' = r + cons, b' = b - dec */
                {
                    int r2 = r + cons;
                    int b2 = b - dec;
                    if (r2 <= Rmax && b2 >= 0 && b2 < BAL_RANGE) {
                        dp_nxt[r2][b2] = max_int(dp_nxt[r2][b2], cur);
                    }
                }

                /* reward: r unchanged, b' = b + inc, reward += rew */
                {
                    int b2 = b + inc;
                    if (b2 >= 0 && b2 < BAL_RANGE) {
                        int val = cur + rew;
                        dp_nxt[r][b2] = max_int(dp_nxt[r][b2], val);
                    }
                }
            }
        }

        /* Swap layers. */
        memcpy(dp_cur, dp_nxt, sizeof(dp_cur));
    }

    /* Compute answer: max reward with used_resource <= Rmax and balance >= Bmin. */
    int ans = 0;
    int b_start = Bmin + BAL_OFF;
    if (b_start < 0) b_start = 0;
    if (b_start >= BAL_RANGE) b_start = BAL_RANGE; /* makes loop empty */

    for (int r = 0; r <= Rmax; r++) {
        for (int b = b_start; b < BAL_RANGE; b++) {
            int v = dp_cur[r][b];
            if (v > ans) ans = v;
        }
    }

    /* If no plan yields positive reward, output 0 (ans already >= 0). */
    printf("%d\n", ans);
    return 0;
}

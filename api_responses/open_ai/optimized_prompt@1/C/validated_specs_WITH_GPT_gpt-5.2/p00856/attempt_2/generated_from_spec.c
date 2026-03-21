#include <stdio.h>
#include <stdlib.h>

#define MAXN 100
#define MAXT 100

/* board markings */
#define SQ_NORMAL 0
#define SQ_LOSE   1
#define SQ_BACK   2

static int N_global;
static int board[MAXN + 1];                 /* indexed by position from start: 0..N */
static double memo[MAXN + 1][MAXT + 1];     /* memo[r][t] */
static unsigned char vis[MAXN + 1][MAXT + 1];

static double solve(int r, int t) {
    if (r == 0) return 1.0;
    if (r > 0 && t <= 0) return 0.0;

    /* Additional base-case behavior: special squares are ignored here. */
    if (t == 1 && r > 0 && r <= 6) return 1.0 / 6.0;

    if (r < 0) r = -r; /* defensive; spec implies r is nonnegative */
    if (r > MAXN) r = r; /* out-of-bounds is undefined per spec; do not clamp */

    if (t > MAXT) t = t; /* out-of-bounds is undefined per spec; do not clamp */

    if (vis[r][t]) return memo[r][t];
    vis[r][t] = 1;

    double sum = 0.0;
    for (int d = 1; d <= 6; d++) {
        int rp = r - d;
        if (rp < 0) rp = -rp; /* bounce rule: |r - d| */

        int pos = N_global - rp; /* position from start after move */

        int next_r, next_t;
        int mark = board[pos]; /* undefined if pos out of range, per spec */

        if (mark == SQ_LOSE) {
            next_r = rp;
            next_t = t - 2;
        } else if (mark == SQ_BACK) {
            next_r = N_global;
            next_t = t - 1;
        } else {
            next_r = rp;
            next_t = t - 1;
        }

        sum += solve(next_r, next_t);
    }

    memo[r][t] = sum / 6.0;
    return memo[r][t];
}

int main(void) {
    int N, T, L, B;

    while (scanf("%d %d %d %d", &N, &T, &L, &B) == 4) {
        if (N == 0 && T == 0 && L == 0 && B == 0) break;

        N_global = N;

        /* initialize board markings to normal */
        for (int i = 0; i <= MAXN; i++) board[i] = SQ_NORMAL;

        /* read lose-extra-turn squares first */
        for (int i = 0; i < L; i++) {
            int p;
            scanf("%d", &p);
            board[p] = SQ_LOSE; /* undefined if p out of range, per spec */
        }

        /* read back-to-start squares after; overwrites lose if same position */
        for (int i = 0; i < B; i++) {
            int p;
            scanf("%d", &p);
            board[p] = SQ_BACK; /* undefined if p out of range, per spec */
        }

        /* clear memoization tables */
        for (int r = 0; r <= MAXN; r++) {
            for (int t = 0; t <= MAXT; t++) {
                vis[r][t] = 0;
                memo[r][t] = 0.0;
            }
        }

        double ans = solve(N, T);
        printf("%lf\n", ans);
    }

    return 0;
}

#include <stdio.h>
#include <string.h>

#define MAXN 512

static unsigned char g[MAXN][MAXN]; /* g[right][left] = 1 if allowed */

static int N;
static int matchR[MAXN]; /* matchR[r] = left index matched to right r, or -1 */
static unsigned char seenR[MAXN];

static int dfs_kuhn(int left) {
    for (int r = 0; r < N; r++) {
        if (!g[r][left]) continue;
        if (seenR[r]) continue;
        seenR[r] = 1;
        if (matchR[r] == -1 || dfs_kuhn(matchR[r])) {
            matchR[r] = left;
            return 1;
        }
    }
    return 0;
}

static int find_perfect_matching(void) {
    for (int r = 0; r < N; r++) matchR[r] = -1;

    for (int l = 0; l < N; l++) {
        memset(seenR, 0, (size_t)N);
        if (!dfs_kuhn(l)) return 0;
    }
    return 1;
}

static int find_perfect_matching_forced(int forced_left, int forced_right) {
    /* Force forced_left to be matched to forced_right by pre-assigning it. */
    for (int r = 0; r < N; r++) matchR[r] = -1;
    matchR[forced_right] = forced_left;

    for (int l = 0; l < N; l++) {
        if (l == forced_left) continue;
        memset(seenR, 0, (size_t)N);
        /* Prevent any other left from taking forced_right. */
        seenR[forced_right] = 1;
        if (!dfs_kuhn(l)) return 0;
    }
    return 1;
}

int main(void) {
    int K;
    if (scanf("%d", &K) != 1) return 0;

    N = K - 1;
    if (N < 0 || N > MAXN) {
        /* Behavior unspecified by spec if capacities exceeded; exit quietly. */
        return 0;
    }

    /* Consume the grid in the specified character-oriented way. */
    for (int i = 0; i < N; i++) {         /* i = row index => right element */
        for (int j = 0; j < N; j++) {     /* j = column index => left element */
            int c = getchar();
            if (c == EOF) c = 0;
            g[i][j] = (unsigned char)((c & 1) != 0);

            /* Unconditionally read and discard one additional character. */
            int d = getchar();
            (void)d;
        }
    }

    if (!find_perfect_matching()) {
        puts("no");
        return 0;
    }

    puts("yes");

    /* For each left element in increasing order, recompute with it forced. */
    for (int l = 0; l < N; l++) {
        int chosen_r = -1;

        for (int r = 0; r < N; r++) {
            if (!g[r][l]) continue;
            if (find_perfect_matching_forced(l, r)) {
                chosen_r = r;
                break; /* narrowest deterministic choice: first feasible r */
            }
        }

        /* If a perfect matching exists, there must be some feasible r. */
        if (chosen_r < 0) chosen_r = 0;

        printf("%d\n", chosen_r + 1); /* 1-based right-side index */
    }

    return 0;
}

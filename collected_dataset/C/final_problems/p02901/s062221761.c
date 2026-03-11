#include <stdio.h>

#define N 12
#define M 1000
#define MAX_A 10000

#define DP_ROW N
#define DP_COL 4096
#define DP_INF (MAX_A * N + 1)

int min(int x, int y) {
    if (x < y) {
        return x;
    } else {
        return y;
    }
}

static unsigned int n, m;
static int as[M];
static unsigned short cs[M]; // if cs[i] has 0x0001, key i can open box 1
static int dp[DP_ROW][DP_COL]; // dp[M][2 ** N]

// assume that row > 0 && col > 0
void print_dp(int row, int col) {
    for (int i = 0; i < row; i++) {
        printf("%010d", dp[i][0]);
        for (int j = 1; j < col; j++) {
            printf(" %010d", dp[i][j]);
        }
        printf("\n");
    }
}

void solve() {
    for (unsigned int i = 0; i < m; i++) {
        unsigned int bits = cs[i];
        for (unsigned int j = 1; j < ((unsigned int) (1 << n)); j++) {
            if (i > 0) {
                dp[i][j] = min(dp[i][j], dp[i-1][j]);
            }
            if (j & bits) {
                if (dp[i][j & (~bits)] + as[i] < dp[i][j]) {
                    dp[i][j] = dp[i][j & (~bits)] + as[i];
                }
            }
        }
    }
}

int main(void) {
    scanf("%d %d", &n, &m);

    // setup prices and cs
    for (unsigned int i = 0; i < m; i++) {
        int l;
        scanf("%d %d", &as[i], &l);
        cs[i] = 0;
        for (int j = 0; j < l; j++) {
            int x;
            scanf("%d", &x);
            cs[i] |= 1 << (x-1);
        }
    }

    // setup dp
    for (int i = 0; i < DP_ROW; i++) {
        dp[i][0] = 0;
        for (int j = 1; j < DP_COL; j++) {
            dp[i][j] = DP_INF;
        }
    }

    solve();
    // print_dp(m, 1 << n);
    // printf("\n");
    printf("%d\n", dp[m-1][(1 << n) - 1] == DP_INF ? -1 : dp[m-1][(1 << n) - 1]);

    return 0;
}

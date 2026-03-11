#include <stdio.h>
#define MAX_ITEM 100

int n, w;
int dp[101][10001];
int value[MAX_ITEM], weight[MAX_ITEM];

void solve(void);
int Max(int a, int b);

int main(void)
{
    int i, j, max = 0;

    scanf("%d %d",&n, &w);

    for (i = 0; i < n; i++) {
        scanf("%d %d",&value[i], &weight[i]);
    }

    solve();

    /*    for (i = 0; i <= n; i++) {
          for (j = 0; j <= w; j++) {
          printf("%d ",dp[i][j]);
          }
          puts("");
          }*/

    for (i = 0; i <= w; i++) {
        if (max < dp[n][i]) {
            max = dp[n][i];
        }
    }
    printf("%d\n",max);

    return 0;
}

void solve(void)
{
    int i, j;

    for (i = 0; i < n; i++) {

        if (weight[i] <= w) {
            dp[i+1][weight[i]] = Max(dp[i][weight[i]], value[i]);
        } else {
            continue;
        }
        for (j = 1; j <= w; j++) {
            if (dp[i][j] || dp[i+1][j]) {
                dp[i+1][j] = Max(dp[i][j], dp[i+1][j]);
                if (j + weight[i] <= w) {
                    dp[i+1][j+weight[i]] = Max(dp[i][j+weight[i]], 
                        Max(dp[i+1][j+weight[i]], dp[i+1][j] + value[i]));
                }
            }
        }

    }

}

int Max(int a, int b)
{
    if (a < b) {
        return b;
    } else {
        return a;
    }
}
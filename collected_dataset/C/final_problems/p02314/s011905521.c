#include<stdio.h>
#include<stdlib.h>

#define INF 2<<27

int dp[50010];
int n, m;
int c[20];

int min(int a, int b)
{
    if(a > b){
        return b;
    }else{
        return a;
    }
}

int intcmp(const void *a, const void *b)
{
    return *(int*)a < *(int*)b;
}

int dfs(int r)
{
    int i;
    for(i = 0; i < m; i++){
        if(r-c[i] < 0){
            continue;
        }else if(dp[r-c[i]] == INF){
            dp[r] = min(dp[r], dfs(r-c[i])+1);
        }else{
            dp[r] = min(dp[r], dp[r-c[i]]+1);
        }
    }
    return dp[r];
}

int main()
{
    int i;
    scanf("%d %d", &n, &m);
    for(i = 0; i < m; i++) scanf("%d", c+i);
    qsort(c, m, sizeof(int), intcmp);

    for(i = 0; i <= n; i++) dp[i] = INF;
    dp[0] = 0; dp[1] = 1;

    printf("%d\n", dfs(n));

    return 0;
}

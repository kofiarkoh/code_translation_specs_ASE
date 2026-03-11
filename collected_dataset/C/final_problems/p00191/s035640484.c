#include <stdio.h>
#include <string.h>

double max(double a, double b)
{
    if (a > b){
        return (a);
    }
    return (b);
}

int main(void)
{
    int i, j, k;
    int n, m;
    double gr[100][100];
    double dp[100][100];
    double ans;
    int upd;
    
    while (1){
        scanf("%d%d", &n, &m);
        
        if (n + m == 0){
            break;
        }
        
        for (i = 0; i < n; i++){
            for (j = 0; j < n; j++){
                scanf("%lf", &gr[i][j]);
            }
        }
        
        memset(dp, 0, sizeof(dp));
        for (i = 0; i < n; i++){
            dp[0][i] = 1.00;
        }
        
        ans = 0.00;
        for (i = 1; i < m; i++){
            for (j = 0; j < n; j++){
                upd = 0;
                for (k = 0; k < n; k++){
                    if (upd == 0){
                        dp[i][j] = dp[i - 1][k] * gr[k][j];
                        upd = 1;
                    }
                    else {
                        dp[i][j] = max(dp[i][j], dp[i - 1][k] * gr[k][j]);
                    }
                }
            }
        }
        
        for (i = 0; i < n; i++){
            ans = max(ans, dp[m - 1][i]);
        }
        
        printf("%.2f\n", ans);
        
    }
    
    return (0);
}
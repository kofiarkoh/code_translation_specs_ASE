#include<stdio.h>
long long max(long long a, long long b){
  return a > b ? a : b;
}

long long min(long long a, long long b){
  return a < b ? a : b;
}
int main(){
        int n=0;
        scanf("%d", &n);
        int a[n];
        for(int i = 0; i < n; i++){
            scanf("%d",&a[i]);
        }

        long long dp[n][n][2]; // dp[i][j] -> x - y value in i to j

        int i=0, j=0;
        for(i = 0; i < n; i++){
            dp[i][i][0] = a[i];
            dp[i][i][1] = -a[i];
        }

        for(i = n-2; i >= 0; i--){
            for(j = i+1; j < n; j++){
                    dp[i][j][0] = max(a[i]+ dp[i+1][j][1],a[j] + dp[i][j-1][1]);
                    dp[i][j][1] = min(-a[i]+ dp[i+1][j][0], -a[j] + dp[i][j-1][0]);
            }
        }
        printf("%lld\n", dp[0][n-1][0]);
}
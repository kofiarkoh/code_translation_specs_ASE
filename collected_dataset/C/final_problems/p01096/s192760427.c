#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#define REP(i,n) for(int i=0; i<n; i++)
int dp[301][301];
int main(void){
    int n,w[300];
    int x;
    int dp[301][301];
    while(scanf("%d",&n),n){
        memset(dp,0,sizeof dp);
        if(n==0) break;
        for(int i=0;i<n;i++){
            scanf("%d",&x);
            w[i]=x;
        }
        for (int i=n-1; i>=0; i--) {
            dp[i][i] = 0;
            //dp[i][i+1] = 0;
            for (int j=i+1; j<n; j++) {
                if(j==i+1 && (abs(w[i] - w[j]) <= 1)) dp[i][j]=2;
                else if (/*i + 1 < j && */(abs(w[i] - w[j]) <= 1) && dp[i+1][j-1] == j-i-1)
                    dp[i][j] = j - i + 1;
                
                for (int k=j; k<n; k++)
                    if(dp[i][k]<dp[i][j-1] + dp[j][k])
                        dp[i][k]=dp[i][j-1] + dp[j][k];
            }
        }
        
        printf("%d\n", dp[0][n-1]);
        /*REP(i,n+1) {
            REP(j,n+1){
                printf("%d ",dp[i][j]);
            }
            printf("\n");
        }*/
    }
    return 0;
}


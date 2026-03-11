#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define max(p,q)((p)>(q)?(p):(q))
int i,j,k;
int row,column,knum;
int r,c,v;
int table[3010][3010];
long long int ret;
long long int dp[3010][3010][4];

int main()
{
    scanf("%d%d%d",&row,&column,&knum);
    for (i=0;i<3010;i++){
        for (j=0;j<3010;j++){
            table[i][j]=0;
        }
    }
    for (i=0;i<3010;i++){
        for (j=0;j<3010;j++){
            for (k=0;k<4;k++){
                dp[i][j][k]=0;
            }
        }
    }
    for (i=0;i<knum;i++){
        scanf("%d%d%d",&r,&c,&v);
        table[r-1][c-1]=v;
    }
    for (i=0;i<row;i++){
        for (j=0;j<column;j++){
            for (k=2;k>=0;k--){
                dp[i][j][k+1]=max(dp[i][j][k+1],dp[i][j][k]+table[i][j]);
            }
            for (k=0;k<4;k++){
                if (i<row-1) dp[i+1][j][0]=max(dp[i+1][j][0],dp[i][j][k]);
                if (j<column-1) dp[i][j+1][k]=max(dp[i][j+1][k],dp[i][j][k]);
            }
        }
    }
    ret=dp[row-1][column-1][0];
    for (i=0;i<4;i++){
        ret=max(ret,dp[row-1][column-1][i]);
    }
    printf("%lld\n",ret);
    return 0;
}

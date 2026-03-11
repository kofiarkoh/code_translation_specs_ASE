#include<stdio.h>
#include<stdlib.h>
#include<math.h>
long long int max(long long int a,long long int b);
int i,j,k;
int R,C,K;
int r,c,v;
int table[3001][3001];
long long int ret;
long long int dp[3001][3001][4];

int main(void)
{

    scanf("%d %d %d",&R,&C,&K);
    for (i=0;i<3001;i++){
        for (j=0;j<3001;j++){
            table[i][j]=0;
        }
    }
    for (i=0;i<3001;i++){
        for (j=0;j<3001;j++){
            for (k=0;k<4;k++){
                dp[i][j][k]=0;
            }
        }
    }
    for (i=0;i<K;i++){
        scanf("%d %d %d",&r,&c,&v);
        table[r-1][c-1]=v;
    }
    for (i=0;i<R;i++){
        for (j=0;j<C;j++){
            for (k=2;k>=0;k--){
                dp[i][j][k+1]=max(dp[i][j][k+1],dp[i][j][k]+table[i][j]);
            }
            for (k=0;k<4;k++){
                if (i<R-1) dp[i+1][j][0]=max(dp[i+1][j][0],dp[i][j][k]);
                if (j<C-1) dp[i][j+1][k]=max(dp[i][j+1][k],dp[i][j][k]);
            }
        }
    }
    ret=dp[R-1][C-1][0];
    for (i=0;i<4;i++){
        ret=max(ret,dp[R-1][C-1][i]);
    }
    printf("%lld",ret);
    return 0;
}

long long int max(long long int a,long long int b){
    if (a<b) return b;
    else return a;
}

#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
int max(const void *a, const void *b){return *(int*)b - *(int*)a;}

int n;
int s;

uint32_t dp[55][55][3000]={0};


int main(void)
{
    scanf("%d",&n);
    scanf("%d",&s);
    dp[0][0][0] = 1;
    for(int i=0;i<n;i++) for(int j=0;j<=n;j++) for(int k=0;k<=s;k++)
    {
        dp[i+1][j+1][k+((j+1)*2)] += dp[i][j][k] % 1000000007 ;
        dp[i+1][j][k+(j*2)] += dp[i][j][k] % 1000000007 ;
        dp[i+1][j][k+(j*2)] += dp[i][j][k] * j % 1000000007 ;
        dp[i+1][j][k+(j*2)] += dp[i][j][k] * j % 1000000007 ; 
        if(j>=1) dp[i+1][j-1][k+((j-1)*2)] += dp[i][j][k] * j * j % 1000000007 ;
    }

    printf("%d\n", dp[n][0][s] % 1000000007 );

    return 0;
}

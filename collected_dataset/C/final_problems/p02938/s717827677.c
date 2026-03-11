#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */

#define MAXN (100000)
#define MOD (1000000007)

uint64_t dp[61][2][2][2];

int main(void)
{
    uint64_t l,r,ans;
    scanf("%ld %ld",&l, &r);

    //dp[桁][L<=X][Y<=R][先頭ビットが出た]
    dp[60][0][0][0] = 1;
    for(int i=59;i>=0;i--)
    {
        int lb = (l>>i)&1;
        int rb = (r>>i)&1;
        for(int j=0;j<2;j++)for(int k=0;k<2;k++)for(int s=0;s<2;s++)
        {
            for(int x=0;x<2;x++)for(int y=0;y<2;y++)
            {
                int nj = j, nk = k, ns = s;
                if(x && !y) continue;
                if(!s && (x!=y)) continue;
                if(!s &&  x && y) ns = 1;
                if(!j && !x && lb) continue;
                if(!j && x && !lb) nj = 1;
                if(!k && y && !rb) continue;
                if(!k && !y && rb) nk = 1;

                dp[i][nj][nk][ns] += dp[i+1][j][k][s];
                dp[i][nj][nk][ns] %= MOD;
            }
        }

    }
    ans = 0;
    for(int j=0;j<2;j++)for(int k=0;k<2;k++)for(int s=0;s<2;s++)
    {
        ans = (ans + dp[0][j][k][s])%MOD;
    }
    printf("%ld\n",ans);
    return 0;
}




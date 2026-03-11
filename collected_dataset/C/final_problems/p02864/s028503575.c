#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
#include <assert.h>
typedef int64_t ll;
typedef uint64_t ull;
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)

#define MAXN (300)
#define MOD (1000000007)

ll h[MAXN+1];
ll dp[MAXN+1][MAXN+1];
int main(void)
{
    ll n,k;
    scanf("%ld %ld",&n,&k);

    for(int i=0;i<n;i++) scanf("%ld",&h[i+1]);
    ll m = n-k;
    if(m==0)
    {
        printf("0\n");
        return 0;
    }
    for(int i=0;i<=n;i++) for(int j=0;j<=m;j++) dp[i][j] = INT64_MAX/10;
    dp[0][0] = 0;
    for(int i=1;i<=n;i++)
    {
        for(int j=1;j<=m;j++)
        {
            for(int l=0;l<i;l++)
            {
                dp[i][j] = min(dp[i][j], dp[l][j-1] + max(0, h[i]-h[l]));
            }
        }
    }

    
    ll ans=INT64_MAX/10;
    for(int i=1;i<=n;i++) ans = min(ans, dp[i][m]);
    printf("%ld\n", ans);
    return 0;
}

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

#define MAXN (2000)
#define MOD (1000000007)


ll dp[MAXN+1][MAXN+1];
ll s[MAXN+1];
ll t[MAXN+1];
int main(void)
{
    ll n,m;
    scanf("%ld %ld",&n,&m);
    for(int i=0;i<n;i++) scanf("%ld",&(s[i]));
    s[n] = 0;
    for(int i=0;i<m;i++) scanf("%ld",&(t[i]));
    t[m] = 0;

    for(int i=0;i<=n;i++) dp[0][i] = 1;

    for(int j=1;j<=m;j++)
    {
        ll p = 0;
        dp[j][0] = 1;
        for(int i=1;i<=n;i++)
        {
            if(s[i-1]==t[j-1])
            {
                p+= dp[j-1][i-1];
                p%=MOD;
            }
            dp[j][i] = dp[j-1][i] + p;
            dp[j][i] %= MOD;
        }
    }

    printf("%ld\n",dp[m][n]);
    return 0;
}

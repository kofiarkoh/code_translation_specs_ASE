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

#define MAXN (1000000000)
#define MOD (1000000007)

ll d[100000];
ll dn[100000];
ll dp[100][100000];
int main(void)
{
    ll n,k;
    scanf("%ld %ld",&n,&k);
    ll dc=1;
    d[0]=1;
    while((n/d[dc-1])!=1)
    {
        d[dc] = (n/(n/d[dc-1]))+1;
        dc++;
    }
    for(int i=0;i<dc-1;i++) dn[i] = d[i+1]-d[i];
    dn[dc-1] = n-d[dc-1]+1;
    for(int i=0;i<dc;i++) dp[0][i] = dn[i];

    for(int i=1;i<k;i++)
    {
        ll s = 0;
        for(int j=dc-1;j>=0;j--)
        {
            s = (s+dp[i-1][(dc-1)-j])%MOD;
            dp[i][j] = s * dn[j] %MOD;
        }
    }
    ll ans=0;
    for(int i=0;i<dc;i++) ans= (ans +dp[k-1][i])%MOD;
    printf("%ld\n",ans);

    return 0;
}

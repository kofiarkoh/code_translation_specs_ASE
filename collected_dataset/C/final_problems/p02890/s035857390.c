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

#define MAXN (300001)
#define MOD (1000000007)

// 指定された要素以上の値が現れる最初の位置のイテレータ
typedef int lb;
int lower_bound(lb* a, lb val, int len)
{
    int l = 0;
    int r = len;
    while(l!=r)
    {
        int m = (l+r)/2;
        if(a[m]<val) l = m+1;
        else r = m;
    }
    return l;
}

int c[MAXN];
int cs[MAXN];
int main(void)
{
    ll n;
    scanf("%ld",&n);
    for(int i=0;i<n;i++)
    {
        ll a;
        scanf("%ld",&a);
        c[a]++;
    }
    qsort(c,MAXN,sizeof(int),acs);
    cs[0] = c[0];
    for(int i=1;i<MAXN;i++) cs[i]=cs[i-1]+c[i];
    
    for(int k=1;k<=n;k++)
    {
        int ans = n/k;
        for(;ans>0;ans--)
        {
            int i = lower_bound(c,ans,MAXN);
            int s = i>0 ? cs[i-1] : 0;
            s += (MAXN-i)*ans;
            if(s>=(ans*k)) break;
        }
        printf("%d\n", ans);
    }
    return 0;
}

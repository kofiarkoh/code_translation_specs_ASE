#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
typedef int64_t ll;
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)

#define MAXN (100000)
#define MOD (998244353)

int64_t a[MAXN];

int main(void)
{
    int n;
    scanf("%d",&n);

    ll x = 0;
    for(int i=0;i<n;i++)
    {
        scanf("%ld",&(a[i]));
        x ^= a[i];
    }
 
    for(int i=0;i<n;i++)
    {
        a[i] &= ~x;
    }

    //掃き出し法
    int r=0;
    for(int i=60;i>=0;i--)
    {
        for(int j=r;j<n;j++)
        {
            if(a[j]&(1L<<i))
            {
                ll t;
                t = a[r];
                a[r] = a[j];
                a[j] = t;
                for(int k=0;k<n;k++)
                {
                    if(k==r) continue;
                    if(a[k]&(1L<<i))
                    {
                        a[k] ^= a[r];
                    }
                }
                r++;
                break;
            }
        }
    }

    ll ans=0;
    for(int i=0;i<n;i++)
    {
        ans ^= a[i];
    }
    ans *= 2;
    ans += x;

    printf("%ld\n", ans);
    return 0;
}

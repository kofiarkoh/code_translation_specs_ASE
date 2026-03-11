#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)

#define MAXN (200000)
#define MOD (998244353)


int main(void)
{
    int n;
    scanf("%d",&n);

    int c = 0;
    int cmax = 0;
    int old = 0;
    for(int i=0;i<n;i++)
    {
        int h;
        scanf("%d",&h);
        if(h <= old)
        {
            c++;
            cmax = max(c,cmax);
        }else
        {
            c = 0;
        }
        old = h;
    }
    printf("%d\n",cmax);

    return 0;
}






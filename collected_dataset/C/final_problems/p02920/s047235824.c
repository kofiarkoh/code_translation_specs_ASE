#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)

#define MAXN (262144)
#define MOD (998244353)

int s[MAXN];

int main(void)
{
    int n;
    scanf("%d",&n);

    for(int i=0;i<(1<<n);i++)
    {
        scanf("%d",&(s[i]));
    }
    qsort(s,1<<n,sizeof(int),des);

    bool check = true;
    for(int step=0;step<n;step++)
    {
        int c = 1<<step;
        for(int i=0;i<c;i++)
        {
            if(s[i]<=s[i+c])
            {
                check = false;
                break;
            }
        }
        if(!check) break;
    }

    printf("%s\n",check?"Yes":"No");
    return 0;
}

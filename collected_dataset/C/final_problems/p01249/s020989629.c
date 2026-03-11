#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 102

typedef struct
{
    char w[MAXN];
    int p;
    int len;
}word;

word uso[MAXN];

int word_cmp(const void* x, const void* y)
{
    return strcmp(((const word*)x)->w, ((const word*)y)->w);
}

int main()
{
    int t=0, n, i, j, k, sum, low, high, mid;
    char exp[MAXN << 1];
    int dp[2002];
    while(scanf("%d", &n) && n)
    {
        for(i=0; i<n && scanf("%s%d", uso[i].w, &uso[i].p); i++)
            uso[i].len = strlen(uso[i].w);
        qsort(uso, n, sizeof(word), word_cmp);
        scanf("%s", exp);
        for(i=sum=0; exp[i]; i+=k, sum += uso[j].p)
        {
            low = 0;
            high = n;
            while(low < high)
            {
                mid = low + high >> 1;
                if (strcmp(uso[mid].w, exp+i) > 0)
                    high = mid;
                else low = mid + 1;
            }
            for(j=low?low-1:0; j<n; j++)
            {
                for(k=0; exp[i+k] && uso[j].w[k]==exp[i+k]; k++);
                if(!uso[j].w[k])
                    break;
            }
        }
        for(i=0, memset(dp, -1, sizeof(dp)), *dp=0; i<n; i++)
            for(j=uso[i].p; j<=sum; j++)
                if(dp[j-uso[i].p] >= 0 && (dp[j] < 0 || dp[j-uso[i].p] + uso[i].len <  dp[j]))
                    dp[j] = dp[j-uso[i].p] + uso[i].len;
        printf("Case %d: %d\n", ++t, dp[sum]);
    }
    return 0;
}
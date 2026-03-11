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

#define MAXN (2621440)
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

    for(int step=0;step<n;step++)
    {
        int c = 1<<step;
        for(int i=0;i<c;i++)
        {
            int j= i+c;
            while(s[i]<=s[j])
            {
                j++;
                if(j==1<<n)
                {
                    printf("No\n");
                    return 0;
                }
            }
            int temp = s[i+c];
            s[i+c] = s[j];
            s[j] = temp;
        }
        qsort(&(s[c*2]),(1<<n)-(c*2),sizeof(int),des);
    }

    printf("%s\n","Yes");
    return 0;
}

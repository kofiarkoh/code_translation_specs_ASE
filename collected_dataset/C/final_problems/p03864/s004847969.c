#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define N 10000
int main()
{
    int i, n, sum, x;
    int a[10000];
    scanf("%d%d", &n, &x);
    for(i=0; i<n; i++)
        scanf("%d", &a[i]);
    sum=0;
    for(i=0; i<n-1; i++)
    {
        int jian = 0;
        if(a[i] + a[i+1] > x)
        {
            jian = a[i] + a[i+1] - x;
            if(a[i+1] >=jian)
                a[i+1]-=jian;
            else
            {
                a[i]=a[i] + a[i+1] - jian;
                a[i+1]=0;
            }
        }
        sum+=jian;
    }
    printf("%d", sum);
    return 0;
}

#include <stdio.h>

int main()
{
    int n,k,i;
    scanf("%d %d",&n,&k);

    long long ans = k;

    for (i = 0; i < n - 1; i++)
    {
        ans *= k - 1;
    }
    printf("%lld",ans);
}
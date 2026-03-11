#include <stdio.h>
#include <math.h>

int main(void)
{
    long long n;
    long long ans = 0;
    scanf("%lld", &n);
    long long inf = sqrtl(n+1)-1;
    for (long long i = 1; i <= inf; i++)
    {
        if(n%i == 0)
        {
            ans +=  n/i - 1;
        }
    }

    printf("%lld\n", ans);
    
    return 0;
}
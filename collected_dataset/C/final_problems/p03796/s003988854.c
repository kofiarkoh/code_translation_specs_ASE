#include <stdio.h>
int main (void)
{
    int i;
    int n;
    long long ans = 1;
    scanf("%d",&n);

    for(i=1;i<=n;i++){
        ans *= i;
        if(ans > (1000000000 + 7))
            ans %= (1000000000 + 7);

        
    }

    printf("%lld\n",ans);

    return 0;
}
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long long int f(long long int,long long int);

int main(void)
{
    long long int n,m,mod=1000000007,ans,x;
    scanf("%lld %lld",&n,&m);
    if(n-m>1||n-m<-1)
    {
        printf("0");
        return 0;
    }
    x=f(n,mod);
    if(n>m) ans=x%mod/n*(x%mod);
    else if(n<m) ans=(x%mod*m)%mod*(x%mod);
    else ans=(x%mod*2)%mod*(x%mod);
    printf("%lld",ans%mod);
    return 0;
}

long long int f(long long int n,long long int mod)
{
    int i;
    long long int x=1;
    for(i=1;i<=n;i++)
    {
        x*=i;
        x%=mod;
    }
    return x;
}



#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long long int f(long long int,long long int,long long int);

int main(void)
{
    long long int n,m,mod=1000000007,ans,x,now=1;
    scanf("%lld %lld",&n,&m);
    if(n-m>1||n-m<-1)
    {
        printf("0");
        return 0;
    }
    x=f(n,mod,now);
    ans=f(m,mod,x);
    if(n==m) ans*=2;
    printf("%lld",ans%mod);
    return 0;
}

long long int f(long long int n,long long int mod,long long int now)
{
    int i;
    for(i=1;i<=n;i++)
    {
        now*=i;
        now%=mod;
    }
    return now;
}



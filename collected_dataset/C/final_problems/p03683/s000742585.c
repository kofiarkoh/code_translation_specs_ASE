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
    x%=mod;
    if(n>m) ans=x/n*x;
    else if(n<m) ans=(x*m%mod)*x;
    else ans=x*x*2;
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



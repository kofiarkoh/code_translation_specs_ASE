#include<stdio.h>

int main(void)
{
    long long n,m;
    long long S,L;
    long long i,k,j;
    scanf("%lld%lld",&n,&m);
    k=m-n-n;
    if(k>0)
    {
        m=m-n-n;
        m=m/4;
        S=n+m;
    }
    else if(k<0)
    {
        S=m/2;
    }
    printf("%lld",S);
    return 0;
}
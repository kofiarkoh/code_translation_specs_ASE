#include<stdio.h>
int main()
{
    long long m,n,sum;
    scanf("%lld%lld",&m,&n);
    if(m<=n/2)
        sum=m+(n-2*m)/4;
    else
        sum=n/2;
    printf("%lld",sum);
    return 0;
}

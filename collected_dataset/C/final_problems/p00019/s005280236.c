#include <stdio.h>
long long unsigned int fac(int n);
int main()
{
    int n;
    long long unsigned x;
    
    scanf("%d",&n);
    x=fac(n);
    printf("%llu\n",x);
    return 0;
}
long long unsigned int fac(int n)
{
    int fact;
    if(n==0)
        return 1;
    else if(n==1)
        return 1;
    else
    {
        fact=n*fac(n-1);
        return fact;
    }
}
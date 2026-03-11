#include<stdio.h>
int main()
{
    long long int a,b,d=1,i;
    scanf("%lld%lld",&a,&b);
    for(i=a;i<=b;i++)
    {
        d=d*i;
    }
    if(d>0)
        printf("Positive");
    else if(d<0)
        printf("Negative");
    else
        printf("Zero");
}

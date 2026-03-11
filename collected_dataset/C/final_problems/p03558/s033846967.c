#include <stdio.h>
#define aaa 1000000
int sum(long int a)
{
    long int b=0;
    while(a!=0)
    {
        b+=a%10;
        a=a/10;
    }
    return b;
}

int main(void)
{
    long int k;
    scanf("%ld",&k);
    while(k%2==0)k=k/2;
    while(k%5==0)k=k/5;
    int ans=sum(k);
    for(int p=1;p<aaa;p++)
    {
        if(ans>sum(k*p))
          ans=sum(k*p);
    }
    printf("%d",ans);
}
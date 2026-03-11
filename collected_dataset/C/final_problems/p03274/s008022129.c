#include<stdio.h>
#include<stdlib.h>
int main()
{
    int n,k,i;
    int t=1000000,p,min;
    scanf("%d %d",&n,&k);
    long long int num[n];
    for(i=0;i<n;i++)
    {
        scanf("%lld",&num[i]);
    }
    for(i=0;i<=n-k;i++)
    {
        p=num[i]-num[i+k-1];
        p=abs(p);
        if(abs(num[i])<abs(num[i+k-1])) min=abs(num[i]);
        else min=abs(num[i+k-1]);
        p=p+min;
        if(p<t) t=p;
    }
    printf("%lld",t);
    return 0;
}


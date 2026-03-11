#include<stdio.h>
#include<string.h>

int main()
{
    long long int a,b,c,d,max,i,j,k,l;
    scanf("%lld %lld %lld %lld",&a,&b,&c,&d);
    i=a*c;
    j=a*d;
    k=b*c;
    l=b*d;
    if(i>=j&&i>=k&&i>=l)
        printf("%lld\n",i);
    else if(j>=i&&j>=k&&j>=l)
        printf("%lld\n",j);
    else if(k>=i&&k>=j&&k>=l)
        printf("%lld\n",k);
    else if(l>=i&&l>=j&&l>=k)
        printf("%lld\n",l);
    return 0;
}

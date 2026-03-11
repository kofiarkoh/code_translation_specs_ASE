#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef long long int int64;

int main()
{
    int64 a,b=0;
    int64 arr[30]={0};
    int64 ans=0;
    int i=0,j;

    scanf("%lld",&a);

    b = a;
    //printf("%lld\n",b);

    if(b%2==1)ans=0;
    else
    {
        while(b>0)
        {
            //printf("b = %lld\n",b);
            b/=5;
            arr[i]=b;
            i++;
        }
        for(j=0;j<i;j++)
        {
            ans+=(arr[j]/2);
        }
    }
    printf("%lld",ans);

}
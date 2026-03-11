#include<stdio.h>
int main(void)
{
    long long int n,x,k,d,i;
    scanf("%lld %lld %lld",&x,&k,&d);
    if(x != 0)
    {
        if(x < 0)
        {
            x = -1 * x;
        }
        if(x - d * k >= 0)
        {
            printf("%lld\n",x - d * k);
            return 0;
        }
        if(x - d * k < 0)
        {
            long long int a;
            a = x / d;
            x = x - d * a;
            if(x < 0)
            {
                x = -1 * x;
            }
            if((k - a) % 2 == 0)
            {
                printf("%lld\n",x);
                return 0;
            }
            else
            {
                printf("%lld\n",d - x);
                return 0;
            }
        }
    }
    if(x == 0)
    {
        if(k % 2 == 0)
        {
            printf("0\n");
            return 0;
        }
        else
        {
            printf("%lld\n",d);
            return 0;
        }
    }
    
    
    return 0;
}
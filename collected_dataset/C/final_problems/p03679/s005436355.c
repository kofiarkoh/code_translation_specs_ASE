#include <stdio.h>
int main()
{
    long long int a,b,x;
    scanf("%lld %lld %lld",&x,&a,&b);
    {
        if( b <= a )
        {
            printf("delicious");
        }
        else if ( b > a + x )
        {
            printf("dangerous");
        }
        else
        {
            printf("safe");
        }
    }
return 0;
}
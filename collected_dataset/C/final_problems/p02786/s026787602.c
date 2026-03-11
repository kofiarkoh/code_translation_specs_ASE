#include <stdio.h>


int main()
{
    unsigned long long ones = 0;
    unsigned long long result;
    unsigned long long i;
    unsigned long long least_one;
    unsigned long long H;
    ones = ~ones;
    scanf("%lld",&H);
    for(i=0 ; i< 8*sizeof(unsigned long long) ;i++)
    {
        if((H>>i)&&1 == 1)
        {
            least_one = i;
        }
    }
    result = (~(ones<<(least_one+1)));
    printf("%llu\n",result);
    return 0;
}
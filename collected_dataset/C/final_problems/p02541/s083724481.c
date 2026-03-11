
#include <stdio.h>

int main()
{
    long long int i, sum = 0, N, x;
    scanf("%lld",&N);
    
    for(i=1; ; i++)
    {
        sum = sum + i; 
        if(sum % N == 0)
        {
            break;
        }
    }
    
    printf("%lld\n",i);
    return 0;
}

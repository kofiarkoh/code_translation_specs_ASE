#include <stdio.h>
#include <math.h>

int main()
{
    long long n,i,j,x;
    scanf("%lld", &n);

    i=sqrt(n);
    while(n%i!=0) i--;
    j=n/i;
    x=(i-1)+(j-1);
    printf("%lld\n", x);
    return 0;
}

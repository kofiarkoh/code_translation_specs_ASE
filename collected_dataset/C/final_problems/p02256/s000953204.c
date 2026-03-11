#include <stdio.h>

int main()
{
    long long int x, y, num1, num2, keep;
    
    scanf("%lld %lld", &x, &y);
    num1 = x; num2 = y;
    while(num2 != 0){
        keep = num2;
        num2 = num1 % num2;
        num1 = keep;
    }    
    printf("%lld\n", num1);
    
    return 0;
}
#include <stdio.h>

int main()
{
    int n, i;
    long long int sum;
    
    while(scanf("%d", &n) != EOF){
    	sum = 0;
        for (i = n; i <= 600 - n; i += n){
            sum += i * i;
        }
        sum *= n;
        printf("%lld\n", sum);
    }
    return 0;
}
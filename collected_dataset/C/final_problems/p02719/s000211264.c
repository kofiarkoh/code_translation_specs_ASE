#include <stdio.h>
 
int main(void)
{
    long long int N, K;
    long long int temp1, temp2;
 
    // input
    scanf("%lld %lld", &N, &K);
 
    temp1 = N % K;
    if (temp1 < 0) {
        temp1 *= -1;
    }
    temp2 = temp1 - K;
    if(temp2 < 0) {
        temp2 *= -1;
    }
    // printf("%lld, %lld\n", temp1, temp2);
    if (temp1 > temp2) {
        printf("%lld\n", temp2);
    } else {
        printf("%lld\n", temp1);
    }
    return 0;
}
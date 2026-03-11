/*
 * ID: 3027
 * Problem: OOIIOII, counting bits in binary
 */

#include <stdio.h>

int main(void)
{
    long n;
    scanf("%ld", &n);

    long k, count;
    for (k = 1, count = 0; n >= k - 1; k *= 2, count++) 
        ;

    printf("%d\n", count - 1);

    return 0;
}


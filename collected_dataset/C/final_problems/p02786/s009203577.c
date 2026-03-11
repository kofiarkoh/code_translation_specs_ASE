#include<stdio.h>

long power(long x, long y)
{
    long i, mult=1;
    for(i=1; i<=y; i++){
        mult *= x;
    }
    return mult;
}

int main()
{
    long n, div, count=0;
    scanf("%ld", &n);
    long x = n;
    do{
        div = x/2;
        x = div;
        count++;
    }while(div!=1);
    //printf("%ld", count);
    printf("%ld", 2*(power(2, count))-1);
    return 0;
}


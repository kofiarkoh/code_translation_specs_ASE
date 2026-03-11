#include <stdio.h>
typedef unsigned long long ULL;

ULL gcd(ULL a, ULL b);
ULL lcm(ULL a, ULL b);

int main(){
    ULL a, b, c, d;
    scanf("%llu %llu %llu %llu", &a, &b, &c, &d);

    b = b - b / c - b / d + b / lcm(c, d);
    a = (a - 1) - (a - 1) / c - (a - 1) / d + (a - 1) / lcm(c, d);

    printf("%llu\n", b - a);
    return 0;
}

ULL gcd(ULL a, ULL b){
    return ((b != 0) ? gcd(b, a % b) : a);
}

ULL lcm(ULL a, ULL b){
    return a * b / gcd(a, b);
}
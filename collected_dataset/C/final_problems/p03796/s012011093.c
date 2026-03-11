#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>

#define MAX 100000
#define M   1000000007

typedef unsigned long long ULL;


ULL training(ULL N) {
    ULL power = 1;
    for(ULL i = 1; i <= N; i++) {
        power *= i;
        power %= M;
    }
    return power;
}

int main(int argc, const char * argv[]) {
    ULL N;
    scanf("%llu", &N);
    if(N < 1 || N > 100000) {
        return 1;
    }
    
    printf("%lld\n", training(N));
    return 0;
}

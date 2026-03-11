#include <stdio.h>
#include <math.h>

int separate(unsigned long long n, int s);

int main(void) {

    unsigned long long num;
    int s;
    int a;

    scanf("%d %llu", &s, &num);
    if ( num == 1 ) {
        puts("1");
    } else if ( s == 1 ) {
        printf("%llu\n", num);
    } else {
        printf("%d\n", separate(num, s));
    }

    return 0;
}

int separate(unsigned long long n, int s) {

    unsigned long long i;
    int c;
    int a = 1;

    for ( i = 2; i <= n; i++ ) {
        for ( c = 0; !(n % i); c++ ) {
            n /= i;
        }
        if ( c/s ) {
            a *= pow(i, (c/s));
        }
    }

    return a;

}
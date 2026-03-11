#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

int64_t test(uint64_t n, uint64_t s);
int main(void) {
    char *p;
    char str[16];
    uint64_t n,s;

    p = fgets(str, 16, stdin);
    n = strtoul(p,0,0);
    p = fgets(str, 16, stdin);
    s = strtoul(p,0,0);

    printf("%ld\n", test(n,s));
//    printf("%ld\n", test(87654UL,30UL));
//    printf("%ld\n", test(87654UL,138UL));
//    printf("%ld\n", test(87654UL,45678UL));
//    printf("%ld\n", test(31415926535UL,1UL));
//    printf("%ld\n", test(1UL,31415926535UL));
//    printf("%ld\n", test(100000000000,1));
    
    return 0;
}

int64_t test(uint64_t n, uint64_t s) {
    uint64_t i,j;
    uint64_t tmp;
    uint64_t sum;
    uint64_t sqrtN;
    uint64_t p;
    int64_t ret = -1;
    if(n == s) {
        return n+1;
    } else if(n < s){
        return -1;
    } else if(s == 1){
        return n;
    }

    sqrtN = (uint64_t)sqrt((double)n) + 1;

    for(i=2UL;i<=sqrtN;i++){
        tmp = n;
        sum = 0;
        while(tmp > 0) {
            sum += (tmp % i);
            tmp /= i;
        }
        if(sum == s){
            ret = i;
            break;
        }
    }
    if(ret == -1){
    for(p=1;p<=sqrtN;p++){
        if((((n-s) % p) == 0)
            && (((n % ((n-s)/p + 1)) + (n-s) + p) == n)){
            //printf("p:%lu\n",p);
            ret = 1 + (n-s)/p;
            break;
        }
   
    }
    }
    
    return ret;
}
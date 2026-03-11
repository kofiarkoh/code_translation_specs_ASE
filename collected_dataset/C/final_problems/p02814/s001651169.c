#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>
#include<string.h>
#include<math.h>
#include<inttypes.h>
#define mprint(x) printf(FORMATCONVERSION(x),(x))
#define FORMATCONVERSION(x) _Generic((x),\
                                    int:"%d",unsigned int:"%u",\
                                    double:"%lf",long long int:"%lld",\
                                    unsigned long long int:"%llu",\
                                    unsigned long : "%lu",\
                                    long int : "%ld",\
                                    char :"%c",char*:"%s")


uint32_t gcd(uint32_t a, uint32_t b) {
  uint32_t v0 = a, v1 = b, v2 = a % b;
  while (v2 != 0) {
    v0 = v1;
    v1 = v2;
    v2 = v0 % v1;
  };
  return v1;
}

unsigned long long int lcm(uint32_t a, uint32_t b) {
  return a * b / gcd(a, b);
}

int main(void){
    int n;
    uint64_t m;
    scanf("%d %"SCNu64,&n,&m);
    uint32_t array[n];
    for(int i = 0;i < n;i++){
        scanf("%" SCNu32,&array[i]);
    }
    if(n == 1){
        unsigned long long int ans = (m / (array[0] / 2) + 1) / 2;
        mprint(ans);
        return 0;
    }
    unsigned long long int  g_num = lcm(array[0],array[1]);
    for(int i = 2;i < n;i++){
        g_num = lcm(g_num,array[i]);
    }

    unsigned long long int ans = (m  / (g_num / 2) + 1) / 2;
    mprint(ans);
    return 0;
}
#include<stdio.h>
#include<math.h>
#include<stdlib.h>

unsigned long long int frac(unsigned long long int a){
    if(a > 1) return a * frac(a-1) % 1000000007;
    return 1;
}

int main(){
    unsigned long long int n,m;
    scanf("%llu %llu",&n,&m);
    if(abs((int)n-(int)m) > 1){
        printf("0\n");
        return 0;
    }
    unsigned long long int N = frac(n);
    unsigned long long int M = frac(m);
    unsigned long long int k = (abs((int)n-(int)m) == 0)? 2 : 1;
    int c = (int)((N * M * k) % 1000000007);
    printf("%d\n",c);
}
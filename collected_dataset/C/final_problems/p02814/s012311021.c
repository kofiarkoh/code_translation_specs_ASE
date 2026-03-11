#include <stdio.h>
#include <stdlib.h>
#include <string.h>

unsigned long long gcd(unsigned long long x, unsigned long long y){
    return y == 0 ? x : gcd(y, x % y);
}

unsigned long long lcm(unsigned long long x,unsigned long long y){
    return x / gcd(x,y) * y;
}

int main(){
    unsigned long long i,n,m,a[100002]={},total=0,p=0;
    scanf("%lld %lld",&n,&m);
    scanf("%lld",&a[0]);
    for (i=1; i<n; i++) scanf(" %lld",&a[i]);
    for (i=0; i<n; i++) a[i]/=2;
    total=lcm(a[0], a[1]);
    for (i=2; i<n; i++) total=lcm(total, a[i]);
    for(i = 0; i < n; i++)
        if(total / a[i] % 2 == 0){
            printf("0\n");
            return 0;
        }
    while ((total*(2*p+1))<=m) p++;
    printf("%lld\n",p);
    return 0;
}

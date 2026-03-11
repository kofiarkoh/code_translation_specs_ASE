#include<stdio.h>
#define MODN 998244353

static inline long modadd(long a,long b){
    return (a+b) % MODN;
}

static inline long modml(long a,long b){
    return (long long)a * b % MODN;
}

static inline long modpow(long n,long ind){
    unsigned long cnt = ind;
    long res = 1;
    while(cnt){
        if(cnt & 1ul)res = modml(res,n);
        n = modml(n,n);
        cnt >>= 1;
    }
    return res;
}

static inline long modinv(long n){
    return modpow(n,MODN - 2);
}

long modfact[200200];

static inline long modnck(long n,long k){
    return modml(modfact[n],modinv(modml(modfact[n-k],modfact[k])));
}

int main(void){
    register int i;
    int n,m,k;
    scanf("%d%d%d",&n,&m,&k);
    modfact[0] = 1;
    for(i = 1;i <= n;i++)modfact[i] = modml(modfact[i-1],i);
    long ans = 0;
    for(i = 0;i <= k;i++)ans = modadd(ans,modml(modml(modpow(m-1,n-i-1),m),modnck(n-1,i)));
    printf("%ld",ans);
}
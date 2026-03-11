#include<stdio.h>
#include<stdlib.h>
#define MOD 1000000007
#define Reg register

#pragma GCC optimize ("O3")
#pragma GCC target ("avx")

int lc(const void *a,const void *b){
    if(*(long*)a - *(long*)b){
        return  *(long*)a < *(long*)b?-1:1;
    }
    return 0;
}

static inline long mdml(long a,long b){
    Reg long long res = (long long)a * b;
    return res % MOD;
}

static inline long mdpow(long num,long ind){
    Reg unsigned long ibit = (unsigned long)ind;
    Reg long res = 1;
    while(ibit){
        if(1lu & ibit)res = mdml(res,num);
        num = mdml(num,num);
        ibit >>= 1;
    }
    return res;
}

static inline long mdadd(long a,long b){
    return (a + b) % MOD;
}

int main(void){
    int n;
    scanf("%d",&n);
    long c[n];
    for(Reg int i = 0;i < n;i++)scanf("%ld",&c[i]);
    qsort(c,n,sizeof(long),lc);
    Reg long ans = 0;
    long tmppow = mdpow(2l,n);
    Reg long tmp2inv = mdpow(2l,MOD-2l);
    Reg long t2n = mdml(tmppow,tmp2inv);
    Reg long t4n = mdml(tmppow,mdml(tmp2inv,tmp2inv));
    Reg int tmpn = n - 1;
    for(Reg int i= 0;i < n;i++)ans = mdadd(ans,mdml(c[i],mdadd(t2n,mdml(t4n,tmpn-i))));
    ans = mdml(ans,tmppow);
    printf("%ld",ans);
}
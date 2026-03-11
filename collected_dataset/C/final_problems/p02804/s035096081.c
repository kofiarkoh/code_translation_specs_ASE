#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int

#define MOD 1000000007
#define MAX 100001
long long int fac[MAX], finv[MAX], inv[MAX];
void MOD_COM_init(){
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for(int i=2; i<MAX; i++){
        fac[i] = fac[i-1]*i % MOD;
        inv[i] = MOD - inv[MOD%i] * (MOD/i) % MOD;
        finv[i] = finv[i-1] * inv[i] % MOD;
    }
}
long long int MOD_combination(long long int n,long long int r){
    if(r<0||n<0){
        return 0;
    }
    if(n < r){
        return 0;
    }
    return (fac[n] * (finv[r] * finv[n-r] % MOD)) % MOD;
}

long long int MOD_pow(long long int a, long long int n){
    long long int res = 1;
    while(n>0){
        if(n&1){
            res = res*a % MOD;
        }
        a = a*a % MOD;
        n >>= 1;
    }
    return res;
}

int cmpAsc(const void* a, const void* b){
    long long int x = *(long long int*)a, y = *(long long int*)b;
    if(x > y){
        return 1;
    }
    if(x < y){
        return -1;
    }
    return 0;
}

int main(){
    int n, k;
    scanf("%d %d", &n, &k);
    lld a[n];
    rep(i, 0, n){
        scanf("%lld", &a[i]);
    }
    qsort(a, n, sizeof(lld), cmpAsc);
    MOD_COM_init();
    lld sum_max = 0;
    lld sum_min = 0;
    rep(i, 0, n){
        sum_max += MOD_combination((lld)(i), (lld)(k-1)) * a[i];
        if(sum_max < 0){
            sum_max += (-1 * sum_max + MOD - 1) / MOD * MOD;
        }
        sum_max %= MOD;
        sum_min += MOD_combination((lld)(n-i-1), (lld)(k-1)) * a[i];
        if(sum_min < 0){
            sum_min += (-1 * sum_min + MOD - 1) / MOD * MOD;
        }
        sum_min %= MOD;
    }
    lld ans = sum_max - sum_min;
    while(ans < 0){
        ans += MOD;
    }
    printf("%lld", ans);

    return 0;
}
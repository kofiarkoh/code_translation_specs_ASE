#include <stdio.h>
#include <stdlib.h>
#define MOD 1000000007
#define MAX 100005

long long fac[MAX], finv[MAX], inv[MAX];

// テーブルを作る前処理
void COMinit(void) {
    fac[0] = fac[1] = 1;
    finv[0] = finv[1] = 1;
    inv[1] = 1;
    for (int i = 2; i < MAX; i++){
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }
};

// 二項係数計算
long long COM(int n, int k){
    if (n < k) return 0;
    if (n < 0 || k < 0) return 0;
    return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD;
};

int int_sort( const void * a , const void * b ) {
  return *(long *)a - *(long *)b;
};


int main(void){
    int long long ans=0,cmb;
    int n,k,i;
    int long a[MAX];
    scanf("%d %d",&n,&k);
    for(i=1;i<=n;i++){
        scanf("%ld",&a[i]);
    };
    qsort(&a[1],n,sizeof(long),int_sort);
    /*for(i=1;i<=n;i++){
        printf("%d ",a[i]);
    };*/
    //putchar('\n');
    COMinit();
    for(i=k;i<=n;i++){
        cmb=COM(i-1,k-1);
        ans+=(a[i]*cmb)% MOD;
        ans%=MOD;
        //printf("a[%d]=%d, a[%d]=%d, f=%d, cmb=%lld, ans=%lld\n",i,a[i],j,a[j],f,cmb,ans);
    };
    for(i=1;i<=n-k+1;i++){
        cmb=COM(n-i,k-1);
        ans-=(a[i]*cmb)% MOD;
        ans%=MOD;
    };
    if(ans<0)ans+=MOD;
    printf("%lld",ans);
    return 0;
}

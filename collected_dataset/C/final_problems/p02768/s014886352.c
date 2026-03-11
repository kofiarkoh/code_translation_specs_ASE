#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#include <time.h>
typedef long long ll;
typedef long double ld;
#define INF (1LL<<60)
#define MOD1 1000000007
/*swap 交換*/
void swap(ll *a, ll *b){ll c;c=*b;*b=*a;*a=c;}
/*2つのうち大きい数を返す*/
ll max2(ll a,ll b){return a>=b?a:b;}
/*2つのうち小さい数を返す*/
ll min2(ll a,ll b){return a>=b?b:a;}
/*絶対値*/
ll ABS(ll a){return a>=0?a:(-a);}
typedef struct{
    ll aa;
    ll bb;
}frequent;
// 小大
int compare(const void *a, const void *b){
    return *(ll *)a>*(ll *)b?1:*(ll *)a<*(ll *)b?-1:0;}
ll digits(ll a, ll b){return a/b?1+digits(a/b,b):1;}
/*nをa進数にしたときの、i桁目の数*/
ll placenai(ll n, ll a, ll i){return i==1?n%a:placenai(n/a,a,i-1);}
ll power2(ll n, ll mod){
    ll i,a[digits(n,2)],ans=1;
    for(i=0;i<digits(n,2);i++){
        if(i==0) a[i]=2;
        else a[i]=a[i-1]*a[i-1]%mod;
        ans*=placenai(n,2,i+1)==1?a[i]:1;
        ans%=mod;
    }
    return ans;
}
ll modfac(ll n, ll mod){
    if(n==1||n==0) return 1;
    ll i,x=1;
    for(i=2;i<=n;i++){
        x*=i%mod;
        x%=mod;
    }
    return x;
}
ll inv[200005],finv[200005];
void makeinv(){
    inv[1]=1;
    finv[0] = finv[1] = 1;
    ll i;
    for(i=2;i<200005;i++){
        inv[i]=MOD1 - inv[MOD1%i] * (MOD1 / i) % MOD1;
        finv[i] = finv[i - 1] * inv[i] % MOD1;
    }
}

int main(void){
    ll n,a,b,i,ans=0,ra,rb,x=1;
    scanf("%lld%lld%lld",&n,&a,&b);
    makeinv();
    for(i=1;i<=b;i++){
        x*=(n-i+1)%MOD1;
        x%=MOD1;
        if(i==a) ra=x;
        if(i==b) rb=x;
    }
    // printf("%lld? %lld %lld ",rb,finv[a],finv[b]);
    ra*=finv[a];
    rb*=finv[b];
    ra%=MOD1;
    rb%=MOD1;
    ans=power2(n,MOD1)-ra-rb-1;
    while(ans<0) ans+=MOD1;
    // printf("%lld %lld %lld ",power2(n,MOD1),ra,rb);
    printf("%lld\n",ans%MOD1);
    return 0;
}

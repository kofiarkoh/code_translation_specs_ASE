#include <stdio.h>
#define rep(i,l,r) for((i)=(l);(i)<(r);(i)++)
#define MOD 1000000007
#define ll long long
ll inv(ll a){
  ll t=1;
  while (a>1){
    t=t*(MOD-MOD/a)%MOD;
    a=MOD%a;
  }
  return t;
}
ll comb(ll n,ll k){
  if (k>n-k) k=n-k;
  ll nu=1,de=1,i;
  rep(i,1,k+1){
    nu=nu*(n+1-i)%MOD;
    de=de*i%MOD;
  }
  return nu*inv(de)%MOD;
}
int main(){
    long long n,m,k,ans=0,i;
    scanf("%lld%lld%lld",&n,&m,&k);
    rep(i,1,n) ans+=i*(n-i)*(m*m);
    rep(i,1,m) ans+=i*(m-i)*(n*n);
    ans*=comb(n*m-2,k-2);
    printf("%lld",ans%MOD);
}
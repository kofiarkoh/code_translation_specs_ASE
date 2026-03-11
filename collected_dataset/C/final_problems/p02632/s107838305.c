#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define ll long long
#define mod 1000000007

char s[1123456];
ll k;

ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}

int main(){
  scanf("%lld",&k);
  scanf("%s",s);

  ll naga=strlen(s);
  ll ans=pom(26,k,mod);
  ll tmp=1;
  for(ll i=1;i<=k;i++){
    tmp = tmp*(i+naga-1)%mod*pom(i,mod-2,mod)%mod;
    ans += tmp*pom(25,i,mod)%mod*pom(26,k-i,mod)%mod;
    ans %= mod;
  }
  
  printf("%lld\n",ans);
  
  return 0;
}

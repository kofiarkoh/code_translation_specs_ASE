#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define FOR(n) for(int i=0;i<n;i++)
#define FORJ(n) for(int j=0;j<n;j++)
#define PRN(n) printf("%d\n",n)
#define PRF(n) printf("%lf\n",n)
#define PRL(n) printf("%lld\n",n)
#define PRS(s) printf("%s\n",s)
#define PRC(c) printf("%c",c)
#define INF 10000000000
#define mod 1000000007
#define X 1000000
typedef long long int ll;
ll finv[X];
ll fac[X];
ll inv[X];
ll u(const void *a, const void *b){return *(ll*)a-*(ll*)b;}
ll d(const void *a, const void *b){return *(ll*)b-*(ll*)a;}
ll z(ll a){
  if(a<0)return -a;
  return a;
}
ll min(ll a,ll b){
  if(a>b)return b;
  return a;
}
ll max(ll a,ll b){
  if(a>b)return a;
  return b;
}
ll gcd(ll a,ll b){
  if(!b)return a;
  return gcd(b,a%b);
}
ll lcm(ll a,ll b){return a*b/gcd(a,b);}
int kt(ll a){
  int sum=0;
  while(a){
    a/=10;
    sum++;
  }
  return sum;
}
int ks(ll a){
  int sum=0;
  while(a){
    sum+=a%10;
    a/=10;
  }
  return sum;
}
ll nCr(int n,int k){
  fac[0]=fac[1]=1;
  finv[0]=finv[1]=1;
  inv[1]=1;
  for (int i=2;i<X;i++){
    fac[i]=fac[i-1]*i%mod;
    inv[i]=mod-inv[mod%i]*(mod/i)%mod;
    finv[i]=finv[i-1]*inv[i]%mod;
  }
  return fac[n] * (finv[k] * finv[n - k] % mod) % mod;
}
int main(void){
  int n;
  scanf("%d",&n);
  ll c,d,ans=-1,sum=0;
  FOR(n){
    scanf("%lld %lld",&c,&d);
    ans+=d;
    d*=c;
    sum+=d;
    ans+=sum/10;
    sum%=10;
  }
  PRN(ans);
  return 0;
}

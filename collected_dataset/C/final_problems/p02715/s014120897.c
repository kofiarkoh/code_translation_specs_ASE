#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define MOD1 1000000007
#define MOD2 998244353
#define LIMIT1 200002
#define LIMIT2 500002
typedef long long ll;
typedef long double ld;
#define rep(i,n) for(i=0;i<n;i++)
#define max(a,b) ((a)>(b) ? (a) : (b))
#define min(a,b) ((a)<(b) ? (a) : (b))
#define zt(a,b) (max(a,b)-min(a,b))
ll gcd(ll a,ll b){return b?gcd(b,a%b):a;}
ll nPr(int n,int r){ll i,result=1;rep(i,r){result*=(ll)(n-i);}return result;}
ll nCr(int n,int r){ll i,result=1; rep(i,min(r,n-r)){result*=(ll)(n-i);result/=(ll)(i+1);}return result;}
#define fact(n) nPr((int)n,(int)n)
#define nHr(n,r) nCr((int)(n+r+1),(int)r)
#define sankaku(x) ((1+x)*x)/2
int dx[8]={1,0,-1,0,1,-1,-1,1};
int dy[8]={0,1,0,-1,1,1,-1,-1};
int upint(const void *a, const void *b) { return *(int *)a < *(int *)b ? -1 : *(int *)a > *(int *)b ? 1 : 0; }
int downint(const void *a, const void *b) { return *(int *)a < *(int *)b ? 1 : *(int *)a > *(int *)b ? -1 : 0; }
int upchar(const void* left, const void* right) {return strcmp((char *)left,(char *)right);}
int downchar(const void* left, const void* right) {return strcmp((char *)right,(char *)left);}
void initialize(){
  
}
ll modpow(ll a, ll n, ll mod) {
    long long res = 1;
    while (n > 0) {
        if (n & 1) res = res * a % mod;
        a = a * a % mod;
        n >>= 1;
    }
    return res;
}
int main(void){
  initialize();
  ll n,k,i,j,result=0,x;
  ll dp[LIMIT2]={0};

  scanf("%lld %lld",&n,&k);
  for(x=k;x>=1;x--){
    dp[x]=modpow((ll)floor(k/x),n,MOD1);
    for(i=2*x;i<=k;i+=x){
      dp[x]-=dp[i];
      if(dp[x]<0) dp[x]+=MOD1;
    }
    //printf("%lld\n",dp[x]);
    result+=dp[x]*x;
    result%=MOD1;
  }
  printf("%lld",result);
  return 0;
}


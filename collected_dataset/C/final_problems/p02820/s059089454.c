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
typedef const void cv;
#define rep(i,n) for(i=0;i<n;i++)
#define max(a,b) ((a)>(b) ? (a) : (b))
#define min(a,b) ((a)<(b) ? (a) : (b))
#define zt(a,b) (max((a),(b))-min((a),(b)))
ll gcd(ll a,ll b){return b?gcd(b,a%b):a;}
ll nPr(int n,int r){ll i,result=1;rep(i,r){result*=(ll)(n-i);}return result;}
ll nCr(int n,int r){ll i,result=1;if(n<r) return 0;rep(i,min(r,n-r)){result*=(ll)(n-i);result/=(ll)(i+1);}return result;}
#define fact(n) nPr((int)(n),(int)(n))
#define nHr(n,r) nCr((int)((n)+(r)+1),(int)(r))
#define sankaku(x) (((x)*((x)+1))/2)
int dx[8]={1,0,-1,0,1,-1,-1,1};
int dy[8]={0,1,0,-1,1,1,-1,-1};
int upint(const void *a, const void *b) { return *(int *)a < *(int *)b ? -1 : *(int *)a > *(int *)b ? 1 : 0; }
int downint(const void *a, const void *b) { return *(int *)a < *(int *)b ? 1 : *(int *)a > *(int *)b ? -1 : 0; }
int upchar(const void* left, const void* right) {return strcmp((char *)left,(char *)right);}
int downchar(const void* left, const void* right) {return strcmp((char *)right,(char *)left);}
ll modpow(ll a, ll n, ll mod) {
  ll res = 1;
  while (n > 0) {
    if (n & 1) res = res * a % mod;
    a = a * a % mod;
    n >>= 1;
  }
  return res;
}
void initialize(){
    
}
int main(void){
  initialize();
  ll n,m,i,j,k,result=0,r,s,p;
  int a[LIMIT2]={0};
  char t[LIMIT1],pc[LIMIT1]={'\0'};
  
  scanf("%lld %lld",&n,&k);
  scanf("%lld %lld %lld",&r,&s,&p);
  scanf("%s",t);
  rep(i,n){
    if(i<k || t[i]!=t[i-k]){
      if(t[i]=='r'){
        strcat(pc,"p");
        result+=p;
      }else if(t[i]=='s'){
        strcat(pc,"r");
        result+=r;
      }else{
        strcat(pc,"s");
        result+=s;
      }
    }else{
      if(t[i]=='r'){
        if(pc[i-k]!='p'){
          strcat(pc,"p");
          result+=p;
        }else{
          strcat(pc," ");
        }
      }else if(t[i]=='s'){
        if(pc[i-k]!='r'){
          strcat(pc,"r");
          result+=r;
        }else{
          strcat(pc," ");
        }
      }else{
        if(pc[i-k]!='s'){
          strcat(pc,"s");
          result+=s;
        }else{
          strcat(pc," ");
        }
      }      
    }
  }
  printf("%lld",result);
  return 0;
}


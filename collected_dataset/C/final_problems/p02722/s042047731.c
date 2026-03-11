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
ll stack[LIMIT2],queue[LIMIT2*10];//stack[0]=count;queue[0]=start,queue[1]=length;
int enqueue(ll n){
    if(queue[0]+queue[1]>=LIMIT2) return -1;
    queue[queue[0]+queue[1]] = n;
    queue[1]++;
    return 1;
}
ll dequeue(){
    ll result;
    if(queue[1]<=0) return -1;
    result = queue[queue[0]];
    queue[0]++;
    queue[1]--;
    if(queue[1]==0){
        queue[0] = 2;
    }
    return result;
}
void initialize(){
    stack[0] = 0;
    queue[0] = 2;
    queue[1] = 0;
}
int main(void){
  initialize();
  ll n,m,k,i,j,result=0,sum=0;
  int a[LIMIT1];
  char s[LIMIT1];

  scanf("%lld",&n);
  for(i=1;i<=sqrtl((ld)(n-1));i++){//n-1の約数
    if((n-1)%i==0){
      if(i>=2&&i!=sqrtl((ld)(n-1))){
        result+=2;
        //printf("%lld %lld ",i,(n-1)/i);
      }else{
        result++;
        //if(i==1) printf("%lld ",n-1);
        //else printf("%lld ",i);
      }
    }
  }
  for(i=1;i<=sqrtl((ld)n);i++){
    if((n-1)%i==0&&i>=2){
    }else{
      if(i>=2){
        k=i;
        j=n;
        while(j%k==0) j/=k;
        if(j%k==1){
          //printf("%lld ",i);
          result++;
        }
      }
      k=n/i;
      j=n;
      while(j%k==0) j/=k;
      if(j%k==1){
        //printf("%lld ",n/i);
        result++;
      }
    }
  }

  printf("%lld",result);
  return 0;
}


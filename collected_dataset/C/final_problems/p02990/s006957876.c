#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#define LL long long
#define INF 1<<31-1
#define MIN(x,y) ((x)<(y)?(x):(y))
#define MAX(x,y) ((x)<(y)?(y):(x))
#define FOR(i,a,n) for(i=a;i<n;i++)
#define MOD 1000000007
#define NUM 100005
void swap(int* a,int* b){int tmp=*a;*a=*b;*b=tmp;}
int GCD(int a,int b){int c=1,tmp=MAX(a,b);b=MIN(a,b);a=tmp;while(c!=0){c=a%b;a=b;b=c;}return a;}
int LCM(int a,int b){return a*b/GCD(a,b);}
int POW(int a,int n){int i;LL x=a;if(n==0)return 1;else if(n==1)return a;else FOR(i,0,n-1){x=x*a;if(x>MOD)x%=MOD;}return (int)x;}
int cmp(const void* a,const void* b){return *(int*)a-*(int*)b;}

int COM(int a,int n){
  int i,k=1;
  LL x=1;
  if(a<n)return 0;
  if(a==n||a==0)return 1;
  if(a-n<n)n=a-n;
  FOR(i,1,n+1){
    x*=(a-i+1);
    while(x>MOD){
      if(k>n)x%=MOD;
      else x/=k++;
    }
  }
  while(k<=n)x/=k++;
  return (int)x;
}

int COMP(int a,int n,int b,int m){
  int i,k=1;
  LL x=COM(a,n);
  if(b<m)return 0;
  if(b==m||b==0)return (int)x;
  if(b-m<m)m=b-m;
  FOR(i,1,m+1){
    x*=(b-i+1);
    while(x>MOD){
      if(k>m)x%=MOD;
      else x/=k++;
    }
  }
  while(k<=m)x/=k++;
  return (int)x;
}

int main() {
  int N,K,i;
  scanf("%d%d",&N,&K);
  LL ans[K];
  FOR(i,0,K){
    ans[i]=COMP(N-K+1,i+1,K-1,i);
    //if(ans[i]>MOD)ans[i]%=MOD;
  }

  FOR(i,0,K)printf("%lld\n",ans[i]);
	return 0;
}

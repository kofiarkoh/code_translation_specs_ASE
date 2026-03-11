#include<stdio.h>

typedef long long int int64;

const int mod=1000000007;

int modPow(int r,int n){
  int t=1;
  int s=r;
  while(n){
    if(n&1) t=(int64)t*s%mod;
    s=(int64)s*s%mod;
    n>>=1;
  }
  return t;
}

void run(void){
  int n,k;
  scanf("%d%d",&n,&k);
  printf("%d\n",modPow(k,n));
}

int main(void){
  run();
  return 0;
}


#include<stdio.h>

typedef long long int int64;

const int mod=1000000007;

int permutation(int n,int k){
  if(k>n) return 0;
  int res=1;
  int i;
  for(i=1;i<=k;i++) res=(int64)res*(n+1-i)%mod;
  return res;
}

void run(void){
  int n,k;
  scanf("%d%d",&n,&k);
  printf("%d\n",permutation(k,n));
}

int main(void){
  run();
  return 0;
}


#include<stdio.h>

typedef long long int int64;

int64 calc(int64 k,int64 a,int64 b){
  if(k<=a) return 1;
  if(a-b<=0) return -1;
  return 1+((k-a)+(a-b)-1)/(a-b)*2;
}

void run(void){
  int64 k,a,b;
  scanf("%lld%lld%lld",&k,&a,&b);
  printf("%lld\n",calc(k,a,b));
}

int main(void){
  run();
  return 0;
}
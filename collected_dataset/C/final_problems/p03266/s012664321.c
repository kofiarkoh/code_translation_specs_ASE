#include<stdio.h>
#include<stdlib.h>

int main(void){

  int N, K;
  scanf("%d %d\n", &N, &K);

  int num=0;
  long long ans=0;

  num = N/K;
  ans = num * num * num;

  if(!(K%2)){
    num = N/(K/2) - N/K;
    ans += num * num * num;
  }

  printf("%lld\n", ans);
  return 0;
}

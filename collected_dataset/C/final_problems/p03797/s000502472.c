#include <stdio.h>

long min(long a,long b){
  if(a < b){
    return a;
  }
  return b;
}

int main(void){
  long N,M;
  scanf("%ld%ld",&N,&M);

  long ans = 0;
  long mini = min(N,M/2);
  ans += mini;
  N -= mini; M -= mini*2;
  ans += M/4;

  printf("%ld\n",ans);
  return 0;
}
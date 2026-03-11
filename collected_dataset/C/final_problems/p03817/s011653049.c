#include<stdio.h>

int main(void){
  long long n;
  scanf("%lld",&n);

  long long ans;
  ans = (n/11) * 2;
  if(n%11 != 0){
    ans ++;
    if(n%11 > 5){
      ans ++;
    }
  }

  printf("%lld\n",ans);

  return 0;
}
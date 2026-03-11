#include<stdio.h>


int main(void){
  long long n;
  scanf("%lld",&n);

  int keta , a = n;
  while(a > 10){
    a /= 10;
    keta ++;
  }

  long long ans = 0;
  long long temp , tas , waru;
  for(int i = 0;i < (1<<keta);i ++){
    tas = n;
    temp = i;
    waru = 10;
    while(temp > 0){
      if(temp%2 == 1){
        ans += tas%waru;
        tas = tas/waru;
        waru = 1;
        //printf("%lld\n",ans);
      }
      temp /= 2;
      waru *= 10;
    }
    ans += tas;
    //printf("%lld\n",ans);
  }

  printf("%lld\n",ans);

  return 0;
}
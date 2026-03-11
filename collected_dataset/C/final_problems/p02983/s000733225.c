#include<stdio.h>

int main(void){
  int l , r;
  scanf("%d %d",&l,&r);
  
  int ans , waru;
  waru = l/2019;
  
  if((waru+1)*2019 <= r){
    ans = 0;
  }
  else{
    ans = (l%2019) * ((l+1)%2019);
  }
  
  printf("%d\n",ans);

  return 0;
}

#include<stdio.h>
int main(void) {
  int N,ans=0;
  scanf("%d",&N);
  do{
    ans += 111;
  }while(ans<N);
  printf("%d",ans);
  return 0;
}
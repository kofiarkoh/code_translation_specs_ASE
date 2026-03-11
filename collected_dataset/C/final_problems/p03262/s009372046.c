#include <stdio.h>

int main(void){
  int num;
  int strt;
  int g;
  int ans;
  int tmp;
  int i;

  scanf("%d %d", &num, &strt);

  scanf("%d", &ans);
  ans -= strt;
  if(ans < 0){
    ans *= -1;
  }
  for(i = 1; i < num; i++){
    scanf("%d", &g);
    g -= strt;
    if(g < 0){
      g *= -1;
    }
    if(ans < g){
      tmp = g;
      g = ans;
      ans = tmp;
    }
    while(ans % g != 0){
      tmp = ans;
      ans = g;
      g = tmp % g;
    }
    ans = g;
  }

  printf("%d\n", ans);

  return 0;
}
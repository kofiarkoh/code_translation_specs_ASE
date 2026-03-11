#include <stdio.h>
 
int main(){
  int a, b, c;
  int ans = 0;
  
  scanf("%d %d %d",&a, &b, &c);
  
  ans = c - (a - b);
  
  if (ans < 0) {
    ans = 0;
  }
  
  printf("%d", ans);
  
  return 0;
}
#include <stdio.h>
int min(int a, int b){
  return (a<b) ? a : b;
}
int main(void){
  int a, b, c, d;
  scanf("%d%d%d%d", &a,&b, &c, &d);
  printf("%d\n", min(a, b) + min(c, d));  
  return 0;
}
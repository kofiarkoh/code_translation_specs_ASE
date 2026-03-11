#include <stdio.h>
int main(void){
  int a,b,c;
  scanf("%d",&a);
  scanf("%d",&b);
  if (a>b) c=a-b;
  if (b>a) c=0;
  printf("%d",c);
  return 0;
}
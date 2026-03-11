#include<stdio.h>
int main(void)
{
  int a, b, c;
  scanf("%d", &a);
  scanf("%d", &b);
  scanf("%d", &c);
  if(a==b){
    printf("%d", c);
  }else if(a==c){
    printf("%d", b);
  }else{
    printf("%d", a);
  }
  return 0;
}
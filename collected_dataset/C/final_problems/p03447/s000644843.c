#include<stdio.h>

int main(){
  int x,a,b;
  scanf("%d",&x);
  scanf("%d",&a);
  scanf("%d",&b);
  x -= a;
  int num;
  num = x/b;
  x -= b*num;
  printf("%d",x);
}

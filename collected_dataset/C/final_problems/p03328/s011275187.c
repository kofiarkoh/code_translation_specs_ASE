#include<stdio.h>

int main(void){
  int t,a,b;
  scanf("%d %d",&a,&b);
  t = (b - a + 1)*(b - a)/2;
  printf("%d",t - b);
}
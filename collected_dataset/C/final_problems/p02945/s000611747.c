#include <stdio.h>

int main(){
  int a,b,m;
  scanf("%d",&a);
  scanf("%d",&b);
  m=a+b;
  if(m<a-b) m=a-b;
  if(m<a*b) m=a*b;
  printf("%d",m);
  return(0);
}
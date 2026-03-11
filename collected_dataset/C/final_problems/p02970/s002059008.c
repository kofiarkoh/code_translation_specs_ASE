#include<stdio.h>
int main(void)
{
  int n,d;
  int a;
  
  scanf("%d %d",&n,&d);
  
  a=n/(2*d+1);
  
  if(n==(2*d+1)*a)
    printf("%d",a);
  else
    printf("%d",a+1);
  
  return 0;
}
#include<stdio.h>

int main()
{
  int a,b,n;
  while(1){
  scanf("%d ",&a);
  scanf("%d",&b);
    if(3<=a&&a<=20&&3<=b&&b<=20)
      break;
  }
  
  if(a>b)
  {
    n=a-1;
    b=n;
  }
  else if(a<b)
  {
   n=b-1;
   a=n;
  }
  
  printf("%d",a+b);
  
  return 0;
}
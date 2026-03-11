#include<stdio.h>
 
int main()
{
  int a,b,c,d;
  scanf("%d %d %d %d",&a,&b,&c,&d);
  a=a+b-c-d;
  if(a>0)printf("Left");
  else if(a==0)printf("Balanced");
  else printf("Right");
  return 0;
}
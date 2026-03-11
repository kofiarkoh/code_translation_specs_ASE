#include<stdio.h>
#include<stdlib.h>
int main()
{
  int a,m,c,a1,a2,a3;
  scanf("%d%d%d",&a,&m,&c);
  a1=a/c;
  a2=m/c;
  if(a%c==0)
    a3=a2-a1+1;
  else
    a3=a2-a1;
  printf("%d",a3);
  
}
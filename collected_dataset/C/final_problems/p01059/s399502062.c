/*
  AOJ 
  title:Gossip
  @kankichi57301
 */
#include <stdio.h>
#include <string.h>
#define max(x,y) (((x)>(y))?(x):(y))

int main()
{
  int a,b,m,n, i,ret;

  scanf("%d %d",&n,&m);
  scanf("%d",&a);
  ret=a-1;

  if(m==1)
    ret=max(ret,n-a);
  else
  {
    for(i=2;i<n;i++)
    {
      scanf("%d",&b);
      ret = max(ret,(b-a)/2);
      a=b;
    }
    scanf("%d",&b);
    ret=max(ret,n-b);
  }
  printf("%d\n",ret);
  return(0);
}


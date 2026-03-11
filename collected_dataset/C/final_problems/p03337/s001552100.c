#include<stdio.h>

int main()
{
  int a,b;
  scanf("%d %d",&a,&b);
  int c=a+b;
  int d=a*b;
  int e=a-b;
  int ans=0;
  if(c<d)
    {
      if(d<e)
	{
	  ans=e;
	}
      else
	{
	  ans=d;
	}
    }
  if(c>d)
    {
      if(c<e)
	{
	  ans=e;
	}
      else
	{
	  ans=c;
	}
    }
  printf("%d\n",ans);
  return 0;
}


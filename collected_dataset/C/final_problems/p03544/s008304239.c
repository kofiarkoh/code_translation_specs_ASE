#include<stdio.h>

int main()
{
  int n;
  
  scanf("%d",&n);
  
  long long l[n];
  int i;
  
  l[0]=2,l[1]=1;
  
  for(i=2;i<=n;i++)
  {
    l[i]=l[i-1]+l[i-2];
  }
  
  printf("%lld\n",l[n]);
  
  return 0;
}
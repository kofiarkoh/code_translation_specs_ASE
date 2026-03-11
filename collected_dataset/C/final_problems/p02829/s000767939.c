#include"stdio.h"
int main()
{
  int n,m,sum1,sum2;
  scanf("%d%d",&n,&m);
  if(n>m)
  {
    sum1=n-m;
    printf("%d\n",sum1);
  }
  else
  {
    sum2=n+m;
    printf("%d\n",sum2);
  }
  return 0;
}
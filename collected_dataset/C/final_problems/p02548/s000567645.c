#include <stdio.h>
int main()
{
  int n;
  scanf("%d",&n);
  int a,b;
  a = 1;
  int cnt = 0;
  while (a <= n)
  {
    b = 1;
    //printf("%d",a);
    while (b <= n / a)
    {
      if (a*b < n)
      {
        //printf("a:b %d:%d\n",a,b);
        cnt++;
      }
      b++;
    }
    a++;
  }
  printf("%d",cnt);
}
#include<stdio.h>

int main(void)
{
  int n, m, i, cnt = 0;
  
  scanf("%d %d", &n, &m);
  
  for(i = 1;i <= n;i++)
  {
    if(i != n-1)
    {
      cnt++;
    }
  }
  
  if(cnt >= m)
  {
    printf("YES\n");
  }
  else
  {
    printf("NO\n");
  }
}
#include <stdio.h>

int main(void)
{
  int a;
  int five = 2, seven = 1;
  int i;
  for (i = 0; i < 3; ++i)
  {
    scanf("%d", &a);
    if (a == 5) five--;
    else if(a == 7) seven--;
  }
  if (!five && !seven) printf("YES\n");
  else printf("NO\n");
  return 0;
}

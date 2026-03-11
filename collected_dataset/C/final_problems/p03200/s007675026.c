#include <stdio.h>
#include <string.h>

int main(void)
{

  char str[10000];
  int Bcount = 0, Wcount = 0;
  int ans = 0;

  scanf("%s", str);

  for (int i = 0; i < strlen(str); i++)
  {
    if (str[i] == 'B')
    {
      Bcount++;
    }
    else
    {
      ans += Bcount;
    }
  }
  printf("%d", ans);

  return 0;
}

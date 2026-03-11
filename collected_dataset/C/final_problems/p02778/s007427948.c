#include <stdio.h>
#include <string.h>

int main(void)
{
  char str[100];
  char p[100];

  scanf("%s", str);
  int num = strlen(str);

  for (int i = 0; i < num; i++)
    p[i] = 'x';
  p[num] = '\0';

  printf("%s\n", p);

  return 0;
}

#include <stdio.h>
#include <string.h>

int main(void)
{
  char s[100001];
  int a = 0;
  int b = 0;
  scanf("%s", s);

  for (int i = 0; i < strlen(s); i++) {
    if (s[i] == '0')
      a++;
    else if (s[i] == '1')
      b++;
  }

  if (a > b)
    printf("%d\n", 2 * b);
  else
    printf("%d\n", 2 * a);
}

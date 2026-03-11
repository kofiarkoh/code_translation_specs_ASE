#include <stdio.h>
int main(void)
{
  char str[4];
  fgets(str,4,stdin);
  if ((str[0] == 'a') || (str[0] == 'i') || (str[0] == 'u') || (str[0] == 'e') || (str[0] == 'o'))
  {
    printf("vowel\n");
  }
  else
  {
    printf("consonant\n");
  }
  return 0;
}

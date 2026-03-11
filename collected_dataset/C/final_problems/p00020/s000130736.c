#include <stdio.h>

int main(void)
{
  char ch[81];
  int i;

  gets(ch);
  for(i = 0; ch[i] != '\0'; ++i)
    {
      if('a' <= ch[i] && ch[i] <= 'z')
	{
	  ch[i] -= 32;
	}
    }
  puts(ch);
  return 0;
}
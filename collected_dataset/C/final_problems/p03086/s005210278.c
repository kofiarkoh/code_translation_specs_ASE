#include <stdio.h>

int main()
{
  char s[11];
  scanf("%s",&s);
  int i;
  int counter;
  counter = 0;
  int max;
  max = 0;
  i = 0;
  while (s[i])
  {
    if (s[i] == 'A' || s[i] == 'G' || s[i] == 'C' || s[i] == 'T')
    {
      counter++;
      if (max < counter)
        max = counter;
    }
    else
      counter = 0;
    i++;
  }
  printf("%d\n",max);
}
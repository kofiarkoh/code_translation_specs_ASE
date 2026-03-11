#include <stdio.h>

int main()
{
  char s[17];
  char a[17] = {"CODEFESTIVAL2016"};
  scanf("%s",&s);
  int i;
  i = 0;
  int counter;
  counter = 0;
  while (s[i])
  {
    if (s[i] != a[i])
      counter++;
    i++;
  }
  printf("%d",counter);
}
#include <stdio.h>
int main()
{
  char S[5];
  scanf("%s",&S);
  int i = 0;
  while (S[i])
  {
    if (S[i] == S[i+1])
    {
      printf("Bad");
      return (0);
    }
    i++;
  }
  printf("Good");
}
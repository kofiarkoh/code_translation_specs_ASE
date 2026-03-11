#include <stdio.h>

int main(void) {
  char t[100000] = {0};
  int i, tlen = 0, count = 0;
  scanf("%s", t);

  for (i = 0; t[i] != '\0'; i++)
    tlen++;
  
  for (i = 0; i < tlen - 1; i++)
    if (t[i] != t[i+1])
      count++;
  
  printf("%d\n", count);

  return 0;
}
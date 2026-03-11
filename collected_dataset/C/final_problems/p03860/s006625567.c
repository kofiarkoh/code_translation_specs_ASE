#include <stdio.h>
#include <stdlib.h>

int main(void) {
  char *s;
  s = (char *)malloc(sizeof(char));
  scanf("AtCoder %s Contest", s);
  printf("A%cC\n", *s);
  free(s);
  return 0;
}
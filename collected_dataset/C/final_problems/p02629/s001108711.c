#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define BUF 12

int main(void) {
  long long int n;
  scanf("%lld", &n);
  char str[BUF];
  memset(str, '\0', BUF);

  int pointer = 0;
  while ((n / 26)) {
    str[pointer] = (char) (n % 26 + 96);
    n /= 26;
    pointer++;
  }
  str[pointer] = (char) (n % 26 + 96);
  n /= 26;
  pointer++;

  for (int i = 0; i < pointer; i++) {
    printf("%c", str[pointer - i - 1]);
  }
  printf("\n");

  return 0;
}

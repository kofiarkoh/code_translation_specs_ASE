#include <stdio.h>

int main(void) {
  int x, a, b;
  scanf("%d%d%d", &x, &a, &b);
  x -= a;
  x -= (x / b) * b;
  printf("%d", x);
  return 0;
}
#include <stdio.h>

int main(void) {

  int n;
  scanf("%d", &n);
  int sum = 0;
  while (n > 0) {
    sum += n%10;
    n /= 10;
  }
  if (sum == 1) {
    sum = 10;
  }
  printf("%d\n", sum);

  return 0;
}
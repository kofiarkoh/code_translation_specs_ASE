#include<stdio.h>

int main(void) {
  int n, k, x = 1;
  scanf("%d %d", &n, &k);

  for (int i = 0; i < n; i++) {
    if (x * 2 < x + k) x *= 2;
    else x += k;
  }

  printf("%d", x);

  return 0;
}

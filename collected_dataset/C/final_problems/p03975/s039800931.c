#include <stdio.h>

int main(void)
{
  int n, a, b, i, t[1000], c = 0;

  scanf("%d %d %d", &n, &a, &b);
  for (i = 0; i < n; i++) {
    scanf("%d", &t[i]);
  }

  for (i = 0; i < n; i++) {
    if (t[i] < a || b <= t[i]) {
      c++;
    }
  }

  printf("%d\n", c);

  return 0;
}

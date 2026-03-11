#include <stdio.h>

int main()
{
  int n;
  int ans = 1;
  scanf("%d", &n);

  while (ans * 2 <= n) {
    ans = ans * 2;
  }

  printf("%d", ans);

  return 0;
}
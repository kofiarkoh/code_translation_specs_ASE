#include <stdio.h>

int main()
{
  int a, b;

  scanf("%d %d", &a, &b);

  if (!(b % a)) {
    printf("%d\n", b/a);
  } else {
    printf("%d\n", b/a + 1);
  }
  return 0;
}


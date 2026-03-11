#include<stdio.h>
#include<math.h>

int main()
{
  int n, d;
  scanf("%d%d", &n, &d);
  printf("%d", (int)ceil((double)n / (d * 2 + 1)));
  return 0;
}
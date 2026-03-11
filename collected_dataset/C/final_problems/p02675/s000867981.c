#include <stdio.h>

int main()
{
  int x;
  scanf("%d", &x);
  x %= 10;
  switch(x) {
  case 3:
    puts("bon");
    break;
  case 0:
  case 1:
  case 6:
  case 8:
    puts("pon");
    break;
  default:
    puts("hon");
    break;
  }
}

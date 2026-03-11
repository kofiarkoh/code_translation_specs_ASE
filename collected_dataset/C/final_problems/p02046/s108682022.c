#include <stdio.h>
#include <stdlib.h>

int main()
{
  int x, y, tmp;
  int ans;
   
  while (scanf("%d %d", &x, &y) != EOF) {
    x = abs(x);
    y = abs(y);
    if (x + y == 1) {
      ans = 1;
    } else if (!(x % 2) && !(y % 2)) {
      ans = x + y;
    } else if ((x % 2) && (y % 2)) {
      ans = x + abs(y - 1);
    } else {
      if (!x || !y) {
        ans = x + y + 1;
      } else {
        ans = x + y;
      }
    }
    printf("%d\n", ans);
  }
  
  return 0;
}


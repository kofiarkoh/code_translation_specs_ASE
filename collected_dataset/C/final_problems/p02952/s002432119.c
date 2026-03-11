#include <stdio.h>

int main() {
  int n, m, dig = 0, ans = 0, x = 1;
  scanf("%d", &n);
  m = n;
  while (m != 0) {
    m /= 10;
    dig++;
  }
  if (dig%2 == 0) {
    for (size_t i = 0; i < dig/2; i++) {
      ans += 9*x;
      x *= 100;
    }
  }else {
    for (size_t i = 0; i < (dig-1)/2; i++) {
      ans += 9*x;
      x *= 100;
    }
    ans += n-x+1;
  }
  printf("%d\n", ans);
  return 0;
}

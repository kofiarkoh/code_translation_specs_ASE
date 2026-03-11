#include <stdio.h>

int judge(int num) {
  int i = 0;

  for (i = 2; i < num; i++) {
    if (num % i == 0) return 0;
  }

  return 1;
}

int main( ) {
  int n;
  int num;
  int ans = 0;
  int i;

  scanf("%d", &n);

  for (i = 0; i < n; i++) {
    scanf("%d", &num);
    ans+=judge(num);
  }

  printf("%d\n", ans);

  return 0;
}
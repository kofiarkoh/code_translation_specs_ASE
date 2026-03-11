#include<stdio.h>
int main() {
  double v, y;
  unsigned long long int n;
  while (scanf("%lf", &v) != EOF) {
    y = v / 19.6 * v;
    if (y - y / 5) n = (int) (y / 5) + 2;
    else n = (int) (y / 5) + 1;
    printf("%llu\n", n);
  }
  return 0;
}
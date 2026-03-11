#include <stdio.h>

int exceedhalf(int a[], int rlimit, int length)
{
  if (rlimit < 0) {
    int x = 1;
    for (int i = 0; i < length; i++) {
      x *= 3;
    }
    return x;
  }
  if (length == 0) return 0;
  return (exceedhalf(&a[1], rlimit - a[0], length - 1)
    + 2 * exceedhalf(&a[1], rlimit, length - 1)) % 998244353;
}

int exactlyhalf(int a[], int rest, int length)
{
  if (rest < 0) return 0;
  if (rest == 0) return 1;
  if (length == 0) return 0;
  return (exactlyhalf(&a[1], rest - a[0], length - 1)
          + exactlyhalf(&a[1], rest, length - 1)) % 998244353;
}

int main()
{
  int n, a[300], sum = 0;
  scanf("%d", &n);
  for (int i = 0; i < n; i++) {
    scanf("%d", &a[i]);
    sum += a[i];
  }
  int limit = (sum - 1) / 2;
  int total = 1;
  for (int i = 0; i < n; i++) {
    total *= 3;
    total %= 998244353;
  }
  total -= 3 * exceedhalf(a, limit, n);
  if (sum % 2 == 0) total += 3 * exactlyhalf(a, sum / 2, n);
  if (total < 0) total += 998244353;
  printf("%d\n", total);
  return 0;
}

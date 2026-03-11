#include <stdio.h>
#include <math.h>

int main(void)
{
  long n, p;
  long m;
  long ans = 1;

  scanf("%ld %ld", &n, &p);

  for (int i=2; (m = pow(i,n)) <= p; i++) {
    if (p % m == 0) {
      ans *= i;
      i = ans;
    }
  }

  printf("%ld\n", ans);
}
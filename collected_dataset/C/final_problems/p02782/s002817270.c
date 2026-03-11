#include <stdio.h>
//#include <stdlib.h>
//#include <string.h>
//#include <stdbool.h>
//#include <limits.h>
//#include <math.h>

#define MOD 1000000007
#define N 1000005 //size of array

long fac[N], inv[N], finv[N];

void com_init()
{
  fac[0] = 1;
  fac[1] = 1;
  finv[0] = 1;
  finv[1] = 1;
  inv[1] = 1;
  for (int i = 2; i < N; i++)
  {
    fac[i] = fac[i - 1] * i % MOD;
    inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
    finv[i] = finv[i - 1] * inv[i] % MOD;
  }
}

long combination(int a, int b)
{
  if (a < b)
    return 0;
  if (a < 0 || b < 0)
    return 0;
  return fac[a] * (finv[b] * finv[a - b] % MOD) % MOD;
}

int main() {
  int r1, r2, c1, c2;
  scanf("%d %d %d %d", &r1, &c1, &r2, &c2);
  com_init();
  long ans = 0;
  for (int i=0; i<=r2; i++) {
    ans += combination(i+c2+1, c2);
    ans %= MOD;
    ans -= combination(i+c1, c1-1);
    ans %= MOD;
  }
  for (int i=0; i<r1; i++) {
    ans += combination(i+c1, c1-1);
    ans %= MOD;
    ans -= combination(i+c2+1, c2);
    ans %= MOD;
  }
  if (ans < 0) ans += MOD;
  printf("%ld\n", ans);
  return 0;
}

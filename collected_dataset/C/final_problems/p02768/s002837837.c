#include <stdio.h>
//#include <stdlib.h>
//#include <string.h>
//#include <stdbool.h>
//#include <limits.h>
//#include <math.h>

#define MOD 1000000007
#define N 200005 //size of array

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

int main() {
  int n, a, b;
  scanf("%d%d%d", &n, &a, &b);
  int k = n;
  long ans = 1, com;
  com_init();
  while (k>10) {
    ans *= 1024;
    ans %= MOD;
    k -= 10;
  }
  while (k>0) {
    ans *= 2;
    ans %= MOD;
    k--;
  }
  ans--;
  com = finv[a];
  for (int i=0; i<a; i++) {
    com *= n-i;
    com %= MOD;
  }
  ans -= com;
  com = finv[b];
  for (int i=0; i<b; i++) {
    com *= n-i;
    com %= MOD;
  }
  ans -= com;
  ans %= MOD;
  if (ans < 0)
    ans += MOD;
  printf("%ld\n", ans);
  return 0;
}

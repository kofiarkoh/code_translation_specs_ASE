#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>
int acs(const void *a, const void *b) { return *(int *)a - *(int *)b; }                            /* 1,2,3,4.. */
int des(const void *a, const void *b) { return *(int *)b - *(int *)a; }                            /* 8,7,6,5.. */
int cmp_char(const void *a, const void *b) { return *(char *)a - *(char *)b; }                     /* a,b,c,d.. */
int cmp_str(const void *a, const void *b) { return strcmp(*(const char **)a, *(const char **)b); } /* aaa,aab.. */
#define min(a, b) (a < b ? a : b)
#define max(a, b) (a > b ? a : b)
#define rep(i, l, r) for (long long int i = l; i < r; i++)
#define MAX 1000001
#define MOD 1000000007

typedef long long int lli;
void swap(lli *a, lli *b)
{
  lli tmp;
  tmp = *a;
  *a = *b;
  *b = tmp;
}

lli modinv(lli a, lli m)
{
  lli b = m, u = 1, v = 0;
  while (b)
  {
    lli t = a / b;
    a -= t * b;
    swap(&a, &b);
    u -= t * v;
    swap(&u, &v);
  }
  u %= m;
  if (u < 0)
    u += m;
  return u;
}
lli combi(lli n, lli k)
{
  lli ans = 1;
  rep(i, 1, n + 1) ans = ans * i % MOD;
  rep(i, 1, k + 1) ans = ans * modinv(i, MOD) % MOD;
  rep(i, 1, n - k + 1) ans = ans * modinv(i, MOD) % MOD;
  return ans;
}

int main(void)
{
  lli x, y;
  scanf("%lld %lld", &x, &y);
  lli n = (x + y) / 3;
  if ((x + y) % 3 != 0)
  {
    printf("0\n");
    return 0;
  }

  int k = min(x - n, y - n);
  if (k < 0)
  {
    printf("0\n");
    return 0;
  }
  lli ans = combi(n, k);
  printf("%lld\n", ans);
  return 0;
}

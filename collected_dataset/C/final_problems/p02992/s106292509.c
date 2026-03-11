#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))
#define SORT(a,num,cmp) qsort((a),(num),sizeof(*(a)),cmp)

int cmp_int (const void *a, const void *b) {
  i32 d = *(i32 *)a - *(i32 *)b;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 len = 1;
  i32 *f = ALLOC (100000, i32);
  i32 i = 1;
  for (; i * i < n; ++i) {
    f[len++] = i;
    f[len++] = n / i;
  }
  if (i * i == n) {
    f[len++] = i;
  }
  SORT (f, len, cmp_int);
  for (i32 i = len - 1; i >= 1; --i) {
    f[i] -= f[i - 1];
  }
  const i32 mod = 1000000007;
  i32 *dp = ALLOC (len, i32);
  i32 *imos = ALLOC (len + 1, i32);
  dp[1] = 1;
  for (i32 i = 1; i <= k; ++i) {
    for (i32 j = 1; j < len; ++j) {
      imos[len - j] = dp[j];
    }
    for (i32 j = len - 1; j >= 1; --j) {
      imos[j] = (imos[j] + imos[j + 1]) % mod;
      dp[j] = (i64) imos[j] * f[j] % mod;
    }
  }
  i64 ans = 0;
  for (i32 i = 1; i < len; ++i) {
    ans += dp[i];
  }
  ans %= mod;
  printf ("%" PRIi64 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

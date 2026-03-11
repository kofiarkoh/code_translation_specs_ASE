#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

void trans (uint64_t *dp, i32 k, i32 a) {
  i32 p = a / 64;
  i32 q = a % 64;
  if (q == 0) {
    for (i32 i = k / 64 + 1; i >= p; --i) {
      dp[i] |= dp[i - p];
    }
  } else {
    for (i32 i = k / 64 + 1; i >= p; --i) {
      uint64_t v = dp[i - p];
      dp[i] |= v << q;
      dp[i + 1] |= v >> (64 - q);
    }
  }
}

i32 calc (i32 *a, i32 n, uint64_t *dp, i32 k) {
  if (n == 1) {
    i32 x = k - 1;
    while (((dp[x >> 6] >> (x & 63)) & 1) == 0) {
      x--;
    }
    return x + a[0] < k ? 1 : 0;
  }
  i32 m = n / 2;
  uint64_t *save = ALLOC (k / 64 + 2, uint64_t);
  memcpy (save, dp, sizeof (uint64_t) * (k / 64 + 2));
  for (i32 i = 0; i < m; ++i) {
    trans (dp, k, a[i]);
  }
  i32 res = calc (a + m, n - m, dp, k);
  memcpy (dp, save, sizeof (uint64_t) * (k / 64 + 2));
  free (save);
  for (i32 i = m; i < n; ++i) {
    trans (dp, k, a[i]);
  }
  res += calc (a, m, dp, k);
  return res;
}

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 *a = ALLOC (n, i32);
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, a + i);
  }
  uint64_t *dp = ALLOC (k / 64 + 2, uint64_t);
  dp[0] = 1;
  i32 ans = calc (a, n, dp, k);
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

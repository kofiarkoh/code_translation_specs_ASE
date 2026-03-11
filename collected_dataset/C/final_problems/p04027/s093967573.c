#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

#define POS(i, j) ((j) * (c + 1) + (i))

void run (void) {
  i32 n, c;
  scanf ("%" SCNi32 "%" SCNi32, &n, &c);
  const i32 m = 400;
  i32 *s = (i32 *) calloc ((c + 1) * (m + 1), sizeof (i32));
  const i32 mod = 1000000007;
  for (i32 i = 1; i <= m; ++i) {
    for (i32 j = 0, t = 1; j <= c; ++j, t = (i64) t * i % mod) {
      s[POS(j, i)] = t;
    }
  }
  for (i32 i = 0; i <= c; ++i) {
    for (i32 j = 1; j <= m; ++j) {
      s[POS(i, j)] = (s[POS(i, j)] + s[POS(i, j - 1)]) % mod;
    }
  }
  i32 *a = (i32 *) calloc (2 * n, sizeof (i32));
  i32 *b = a + n;
  for (i32 i = 0; i < 2 * n; ++i) {
    scanf ("%" SCNi32, a + i);
  }
  i32 *dp = (i32 *) calloc (c + 1, sizeof (i32));
  dp[0] = 1;
  for (i32 i = 0; i < n; ++i) {
    for (i32 j = c; j >= 0; --j) {
      i32 local = 0;
      for (i32 k = 0; k <= j; ++k) {
        local = (local + (i64) (s[POS(k, b[i])] + mod - s[POS(k, a[i] - 1)]) * dp[j - k]) % mod;
      }
      dp[j] = local;
    }
  }
  printf ("%" PRIi32 "\n", dp[c]);
}

int main (void) {
  run();
  return 0;
}

#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

const i32 mod = 1000000007;

#define POS(i, j, k) ((((i) & 1) * (n + 1) + (j)) * (n + 1) + (k))

void run (void) {
  i32 n, m;
  scanf ("%" SCNi32 "%" SCNi32, &n, &m);
  i32 *dp = (i32 *) calloc (2 * (n + 1) * (n + 1), sizeof (i32));
  dp[POS(0, n, 0)] = 1;
  for (i32 i = 1; i <= m; ++i) {
    for (i32 j = 1; j <= n; ++j) {
      for (i32 k = 0; k <= n - j; ++k) {
        i64 local = (i64) j * dp[POS(i - 1, j + k, 0)];
        if (k > 0) local += (i64) k * dp[POS(i - 1, j, k)];
        if (n - j - k > 0) local += (i64) (n - j - k) * dp[POS(i - 1, j, k + 1)];
        dp[POS(i, j, k)] = local % mod;
      }
    }
  }
  printf ("%" PRIi32 "\n", dp[POS(m, 1, 0)]);
}

int main (void) {
  run();
  return 0;
}

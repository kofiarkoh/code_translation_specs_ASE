#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  if (n < k) {
    puts("0");
    return;
  }
  n -= k;
  const i32 mod = 1000000007;
  i32 *dp = (i32 *) calloc (n + 1, sizeof (i32));
  dp[0] = 1;
  for (i32 i = 1; i <= k; ++i) {
    for (i32 j = i; j <= n; ++j) {
      dp[j] += dp[j - i];
      dp[j] %= mod;
    }
  }
  printf ("%" PRIi32 "\n", dp[n]);
}

int main (void) {
  run();
  return 0;
}


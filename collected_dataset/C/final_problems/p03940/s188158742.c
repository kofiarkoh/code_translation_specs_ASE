#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

#define MIN(a,b) ((a) < (b) ? (a) : (b))

void run (void) {
  i32 n, e, t;
  scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &n, &e, &t);
  i32 *x = (i32 *) calloc (n + 2, sizeof (i32));
  for (i32 i = 1; i <= n; ++i) {
    scanf ("%" SCNi32, x + i);
  }
  x[n + 1] = e;
  i64 *dp = (i64 *) calloc (n + 1, sizeof (i64));
  i64 min = (i64) n * t + e;
  for (i32 i = 1, j = 1; i <= n; ++i) {
    dp[i] = dp[i - 1] + x[i] - x[i - 1] + t;
    for (; 2 * (x[i] - x[j]) >= t; ++j) {
      min = MIN(min, dp[j - 1] + x[j] - x[j - 1] - (i64) 3 * x[j]);
    }
    dp[i] = MIN(dp[i], min + (i64) 3 * x[i]);
    if (j < i) {
      i64 p = dp[j - 1] + x[j] - x[j - 1];
      p += t;
      p += x[i] - x[j];
      dp[i] = MIN(dp[i], p);
    }
  }
  printf("%" PRIi64 "\n", dp[n] + e - x[n]);
}

int main (void) {
  run();
  return 0;
}

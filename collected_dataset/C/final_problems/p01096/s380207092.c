#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))
#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

#define POS(l, r) ((l) * (n + 1) + (r))

void run (void) {
  for (;;) {
    i32 n;
    scanf ("%" SCNi32, &n);
    if (n == 0) break;
    i32 *a = ALLOC (n, i32);
    for (i32 i = 0; i < n; ++i) {
      scanf ("%" SCNi32, a + i);
    }
    i32 *dp = ALLOC ((n + 1) * (n + 1), i32);
    for (i32 len = 2; len <= n; ++len) {
      for (i32 l = 0; l <= n - len; ++l) {
        i32 r = l + len;
        i32 max = 0;
        for (i32 m = l + 1; m < r; ++m) {
          max = MAX (max, dp[POS(l, m)] + dp[POS(m, r)]);
        }
        for (i32 m = l + 1; m < r; ++m) {
          if (dp[POS(l + 1, m)] == m - l - 1 && ABS (a[l] - a[m]) <= 1) {
            max = MAX (max, m + 1 - l + dp[POS(m + 1, r)]);
          }
        }
        dp[POS(l, r)] = max;
      }
    }
    printf ("%" PRIi32 "\n", dp[POS(0, n)]);
  }
}

int main (void) {
  run();
  return 0;
}


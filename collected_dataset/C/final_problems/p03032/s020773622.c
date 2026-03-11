#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

typedef int32_t i32;
typedef int64_t i64;

#define MAX(a,b) ((a) > (b) ? (a) : (b))
#define MIN(a,b) ((a) < (b) ? (a) : (b))
#define ABS(a) ((a) > (0) ? (a) : -(a))

#define POS(i, j, l) (((i) * n + (j)) * n + (l))

i32 inf = -1000000000 - 1;
i32 calc (i32 k, i32 l, i32 r, i32 *v, i32 *dp, i32 n) {
  if (k < 0) return inf;
  if (k == 0) {
    return 0;
  }
  if (l > r) return inf;
  if (dp[POS(k, l, r)] != inf) return dp[POS(k, l, r)];
  i32 max = 0;
  i32 x = calc (k - 1, l, r, v, dp, n);
  max = MAX(max, x);
  x = calc (k - 1, l + 1, r, v, dp, n) + v[l];
  max = MAX(max, x);
  x = calc (k - 1, l, r - 1, v, dp, n) + v[r];
  max = MAX(max, x);
  x = calc (k - 2, l + 1, r, v, dp, n);
  max = MAX(max, x);
  x = calc (k - 2, l, r - 1, v, dp, n);
  max = MAX(max, x);
  dp[POS(k, l, r)] = max;
  return max;
}

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 *v = (i32 *) calloc (n, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, v + i);
  }
  i32 *dp = (i32 *) calloc ((k + 1) * n * n, sizeof (i32));
  for (i32 i = 0; i < (k + 1) * n * n; ++i) dp[i] = inf;
  i32 ans = calc (k, 0, n - 1, v, dp, n);
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

#define MIN(a,b) ((a) < (b) ? (a) : (b))

typedef struct node {
  i64 n;
  i64 min;
} node;

i64 eval (i64 n, i64 a, i64 r) {
  if (n <= a + r) return n;
  i64 x = n;
  i64 y = r + a + eval ((n + r - 1) / r, a, r);
  return MIN(x, y);
}

void run (void) {
  i64 n, a;
  scanf ("%" SCNi64 "%" SCNi64, &n, &a);
  node dp[64];
  i32 len = 0;
  dp[len++] = (node) {n, n};
  for (i64 i = 2; i * i <= n; ++i) {
    for (i32 j = 0; j < len; ++j) {
      i64 x = eval (dp[j].n, a, i);
      if (dp[j].min <= x) continue;
      i64 d = dp[j].min - x;
      for (i32 k = j - 1; k >= 0; --k) dp[k].min -= d;
      node q[64];
      i64 t = dp[j].n;
      i32 top = 0;
      while (t > a + i) {
	q[top++].n = t;
	t = (t + i - 1) / i;
      }
      q[top++] = (node) {t, t};
      for (i32 k = top - 2; k >= 0; --k) {
	q[k].min = MIN(q[k].n, i + a + q[k + 1].min);
      }
      i32 len = j;
      for (i32 k = 0; k < top; ++k) {
	dp[len++] = q[k];
	if (q[k].n == q[k].min) break;
      }
      break;
    }
  }
  printf ("%" PRIi64 "\n", dp[0].min);
}

int main (void) {
  run();
  return 0;
}

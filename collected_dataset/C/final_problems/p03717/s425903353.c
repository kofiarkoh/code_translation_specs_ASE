#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>

typedef int32_t i32;

typedef struct cond {
  i32 l, r, c;
} node;

int cmp (const void *a, const void *b) {
  i32 d = ((node *)a)->r - ((node *)b)->r;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

#define POS(i, j, k) ((((i) & 1) * n + (j)) * n + (k))

void run (void) {
  i32 n, m;
  scanf ("%" SCNi32 "%" SCNi32, &n, &m);
  node *p = (node *) calloc (m, sizeof (node));
  for (i32 i = 0; i < m; ++i) {
    i32 l, r, c;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &l, &r, &c);
    p[i] = (node) {l, r, c};
  }
  qsort (p, m, sizeof (node), cmp);
  const i32 mod = 1000000007;
  i32 *dp = (i32 *) calloc (2 * n * n, sizeof (i32));
  dp[POS(0, 0, 0)] = 1;
  for (i32 i = 1, q = 0; i <= n; ++i) {
    memset (dp + POS(i, 0, 0), 0, sizeof (i32) * n * n);
    for (i32 j = 0; j < i; ++j) {
      for (i32 k = 0; k <= j; ++k) {
        dp[POS(i, j, k)] = (dp[POS(i, j, k)] + dp[POS(i - 1, j, k)]) % mod;
        dp[POS(i, i - 1, k)] = (dp[POS(i, i - 1, k)] + dp[POS(i - 1, j, k)]) % mod;
        dp[POS(i, i - 1, j)] = (dp[POS(i, i - 1, j)] + dp[POS(i - 1, j, k)]) % mod;
      }
    }
    for (; q < m && p[q].r == i; ++q) {
      for (i32 j = 0; j < i; ++j) {
        for (i32 k = 0; k <= j; ++k) {
          i32 c = 1 + (j >= p[q].l ? 1 : 0) + (k >= p[q].l ? 1 : 0);
          if (c != p[q].c) dp[POS(i, j, k)] = 0;
        }
      }
    }
  }
  i32 ans = 0;
  for (i32 i = 0; i < n; ++i) {
    for (i32 j = 0; j <= i; ++j) {
      ans = (ans + dp[POS(n, i, j)]) % mod;
    }
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

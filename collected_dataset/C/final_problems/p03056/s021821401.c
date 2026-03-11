#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

i32 count (i32 a, i32 b, i32 c, i32 d, i32 **cnt) {
  return cnt[a][c] - cnt[b][c] - cnt[a][d] + cnt[b][d];
}

i32 max (i32 a, i32 b) {
  return a > b ? a : b;
}

i32 min (i32 a, i32 b) {
  return a < b ? a : b;
}

i32 calc (i32 a, i32 b, i32 c, i32 d, i32 **cnt, i32 ****dp) {
  if (dp[a][b][c][d] != 0) return dp[a][b][c][d];
  i32 v = count (a, b, c, d, cnt);
  if (v == 0 || v == (b - a) * (d - c)) return 0;
  i32 local = (b - a) * (d - c);
  if (b - a > 1) {
    for (i32 i = a + 1; i < b; ++i) {
      local = min (local, max (calc (a, i, c, d, cnt, dp), calc (i, b, c, d, cnt, dp)) + 1);
    }
  }
  if (d - c > 1) {
    for (i32 i = c + 1; i < d; ++i) {
      local = min (local, max (calc (a, b, c, i, cnt, dp), calc (a, b, i, d, cnt, dp)) + 1);
    }
  }
  dp[a][b][c][d] = local;
  return local;
}

void run (void) {
  i32 h, w;
  scanf ("%" SCNi32 "%" SCNi32, &h, &w);
  char *s = (char *) calloc (h * w + 1, sizeof (char));
  for (i32 i = 0; i < h; ++i) {
    scanf ("%s", s + i * w);
  }
  i32 **cnt = (i32 **) calloc (h + 1, sizeof (i32 *));
  for (i32 i = 0; i <= h; ++i) {
    cnt[i] = (i32 *) calloc (w + 1, sizeof (i32));
  }
  for (i32 i = 0; i < h; ++i) {
    for (i32 j = 0; j < w; ++j) {
      cnt[i][j] = s[i * w + j] == '#' ? 1 : 0;
    }
  }
  for (i32 i = h - 1; i >= 0; --i) {
    for (i32 j = w - 1; j >= 0; --j) {
      cnt[i][j] += cnt[i + 1][j] + cnt[i][j + 1] - cnt[i + 1][j + 1];
    }
  }
  i32 ****dp = (i32 ****) calloc (h, sizeof (i32 ***));
  for (i32 i = 0; i < h; ++i) {
    dp[i] = (i32 ***) calloc (h - i, sizeof (i32 **)) - i - 1;
    for (i32 j = i + 1; j <= h; ++j) {
      dp[i][j] = (i32 **) calloc (w, sizeof (i32 *));
      for (i32 k = 0; k < w; ++k) {
        dp[i][j][k] = (i32 *) calloc (w - k, sizeof (i32)) - k - 1;
      }
    }
  }
  i32 ans = calc (0, h, 0, w, cnt, dp);
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

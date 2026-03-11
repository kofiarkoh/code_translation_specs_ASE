#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;
typedef uint8_t u8;

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  u8 **win = (u8 **) calloc (n, sizeof (u8 *));
  for (i32 i = 0; i < n; ++i) {
    win[i] = (u8 *) calloc (n, sizeof (u8));
  }
  char *s = (char *) calloc (n + 1, sizeof (char));
  for (i32 i = 1; i < n; ++i) {
    scanf ("%s", s);
    for (i32 j = 0; j < i; ++j) {
      if (s[j] == '0') {
	win[j][i] = 1;
      } else {
	win[i][j] = 1;
      }
    }
  }
  u8 **dpL = (u8 **) calloc (n, sizeof (u8 *));
  u8 **dpR = (u8 **) calloc (n, sizeof (u8 *));
  for (i32 i = 0; i < n; ++i) {
    dpL[i] = (u8 *) calloc (n, sizeof (u8));
    dpR[i] = (u8 *) calloc (n, sizeof (u8));
    dpL[i][i] = dpR[i][i] = 1;
  }
  for (i32 len = 2; len <= n; ++len) {
    for (i32 l = 0; l + len - 1 < n; ++l) {
      i32 r = l + len - 1;
      for (i32 k = l + 1; k <= r && !dpL[r][l]; ++k) {
	dpL[r][l] = win[l][k] & dpR[l + 1][k] & dpL[r][k];
      }
      for (i32 k = l; k <= r - 1 && !dpR[l][r]; ++k) {
	dpR[l][r] = win[r][k] & dpR[l][k] & dpL[r - 1][k];
      }
    }
  }
  i32 ans = 0;
  for (i32 i = 0; i < n; ++i) {
    ans += dpR[0][i] & dpL[n - 1][i];
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

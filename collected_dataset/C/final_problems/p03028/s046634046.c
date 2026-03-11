#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;
typedef uint8_t u8;

#define POS(i, j) ((i) * n + (j))

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  u8 *win = (u8 *) calloc (n * n, sizeof (u8));
  char *s = (char *) calloc (n + 1, sizeof (char));
  for (i32 i = 1; i < n; ++i) {
    scanf ("%s", s);
    for (i32 j = 0; j < i; ++j) {
      if (s[j] == '0') {
        win[POS(j, i)] = 1;
      } else {
        win[POS(i, j)] = 1;
      }
    }
  }
  u8 *dpL = (u8 *) calloc (n * n, sizeof (u8));
  u8 *dpR = (u8 *) calloc (n * n, sizeof (u8));
  for (i32 i = 0; i < n; ++i) {
    dpL[POS(i, i)] = dpR[POS(i, i)] = 1;
  }
  for (i32 len = 2; len <= n; ++len) {
    for (i32 l = 0; l + len - 1 < n; ++l) {
      i32 r = l + len - 1;
      for (i32 k = l + 1; k <= r; ++k) {
        if (win[POS(l, k)] && dpR[POS(l + 1, k)] && dpL[POS(k, r)]) {
          dpL[POS(l, r)] = 1;
          break;
        }
      }
      for (i32 k = l; k <= r - 1; ++k) {
        if (win[POS(r, k)] && dpR[POS(l, k)] && dpL[POS(k, r - 1)]) {
          dpR[POS(l, r)] = 1;
          break;
        }
      }
    }
  }
  i32 ans = 0;
  for (i32 i = 0; i < n; ++i) {
    ans += dpR[POS(0, i)] & dpL[POS(i, n - 1)];
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

uint8_t** allocate (i32 h, i32 w) {
  uint8_t **a = ALLOC (h, uint8_t *);
  uint8_t *buf = ALLOC (h * w, uint8_t);
  for (i32 i = 0; i < h; ++i) {
    a[i] = buf;
    buf += w;
  }
  return a;
}

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  uint8_t **win = allocate (n, n);
  char *s = ALLOC (n, char);
  for (i32 i = 1; i < n; ++i) {
    scanf ("%s", s);
    for (i32 j = 0; j < i; ++j) {
      win[i][j] = s[j] == '1' ? 1 : 0;
      win[j][i] = win[i][j] ^ 1;
    }
  }
  uint8_t **dp_l = allocate (n, n);
  uint8_t **dp_r = allocate (n, n);
  for (i32 i = 0; i < n; ++i) {
    dp_l[i][i] = dp_r[i][i] = 1;
  }
  for (i32 len = 2; len <= n; ++len) {
    for (i32 l = 0; l <= n - len; ++l) {
      i32 r = l + len - 1;
      for (i32 i = l + 1; i <= r; ++i) {
        dp_l[r][l] |= win[l][i] & dp_r[l + 1][i] & dp_l[r][i];
      }
      for (i32 i = l; i < r; ++i) {
        dp_r[l][r] |= win[r][i] & dp_r[l][i] & dp_l[r - 1][i];
      }
    }
  }
  i32 ans = 0;
  for (i32 i = 0; i < n; ++i) {
    ans += dp_r[0][i] & dp_l[n - 1][i];
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run();
  return 0;
}

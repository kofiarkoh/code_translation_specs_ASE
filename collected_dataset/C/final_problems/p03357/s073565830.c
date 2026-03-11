#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

void add (i32 *bit, i32 x, i32 v) {
  i32 n = bit[0];
  for (i32 i = x; i <= n; i += i & -i) {
    bit[i] += v;
  }
}

i32 get_sum (i32 *bit, i32 x) {
  i32 sum = 0;
  for (i32 i = x; i > 0; i -= i & -i) {
    sum += bit[i];
  }
  return sum;
}

void init_bit (i32 *bit) {
  i32 n = bit[0];
  for (i32 i = 1; i <= n; ++i) {
    bit[i] = 0;
  }
}

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  i32 *w = ALLOC (n + 1, i32);
  i32 *b = ALLOC (n + 1, i32);
  for (i32 i = 1; i <= 2 * n; ++i) {
    char s[5];
    i32 a;
    scanf ("%s%" SCNi32, s, &a);
    if (s[0] == 'W') {
      w[a] = i;
    } else {
      b[a] = i;
    }
  }
  i32 *bit = ALLOC (2 * n + 1, i32);
  bit[0] = 2 * n;
  i32 *dp = ALLOC (n + 1, i32);
  for (i32 i = 1; i <= n; ++i) {
    add (bit, b[i], 1);
    dp[i] = dp[i - 1] + (i - get_sum (bit, b[i]));
  }
  for (i32 i = 1; i <= n; ++i) {
    init_bit (bit);
    dp[0] = 0;
    for (i32 j = 1; j <= i; ++j) {
      add (bit, w[j], 1);
      dp[0] += j - get_sum (bit, w[j]);
    }
    for (i32 j = 1; j <= n; ++j) {
      add (bit, b[j], 1);
      i32 x = dp[j - 1] + (i + j - get_sum (bit, b[j]));
      i32 y = dp[j] + (i + j - get_sum (bit, w[i]));
      dp[j] = x < y ? x : y;
    }
  }
  printf ("%" PRIi32 "\n", dp[n]);
}

int main (void) {
  run();
  return 0;
}

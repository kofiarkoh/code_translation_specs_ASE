#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

const i32 mod = 1000000007;

i32 comb (i32 n, i32 k) {
  if (!(0 <= k && k <= n)) return 0;
  i32 a = 1;
  i32 b = 1;
  for (i32 i = 1; i <= k; ++i) {
    a = (i64) a * (n + 1 - i) % mod;
    b = (i64) b * i % mod;
  }
  while (b > 1) {
    a = (i64) a * (mod - mod / b) % mod;
    b = mod % b;
  }
  return a;
}

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 ans = comb (k, n);
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run ();
  return 0;
}


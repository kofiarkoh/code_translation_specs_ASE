#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

const i32 mod = 1000000007;

i32 mod_pow (i32 r, i32 n) {
  i32 t = 1;
  i32 s = r;
  while (n > 0) {
    if (n & 1) t = (i64) t * s % mod;
    s = (i64) s * s % mod;
    n >>= 1;
  }
  return t;
}

i32 comb (i32 n, i32 k) {
  if (!(0 <= k && k <= n)) return 0;
  i32 a = 1;
  i32 b = 1;
  for (i32 i =1; i <= k; ++i) {
    a = (i64) a * (n + 1 - i) % mod;
    b = (i64) b * i % mod;
  }
  return (i64) a * mod_pow (b, mod - 2) % mod;
}

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i32 ans = 0;
  for (i32 i = k, sign = 1; i >= 1; --i, sign *= -1) {
    ans = (ans + mod + (i64) sign * comb (k, i) * mod_pow (i, n) % mod) % mod;
  }
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run ();
  return 0;
}


#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

const i32 mod = 998244353;

i32 inv (i32 a) {
  i32 t = 1;
  while (a > 1) {
    t = (i64) t * (mod - mod / a) % mod;
    a = mod % a;
  }
  return t;
}

i32 *fact = NULL;
i32 *iFact = NULL;
void init_fact (const i32 n) {
  fact = (i32 *) calloc (n + 1, sizeof (i32));
  fact[0] = 1;
  for (i32 i = 1; i <= n; ++i) {
    fact[i] = (i64) i * fact[i - 1] % mod;
  }
  iFact = (i32 *) calloc (n + 1, sizeof (i32));
  iFact[n] = inv (fact[n]);
  for (i32 i = n - 1; i >= 0; --i) {
    iFact[i] = (i64) (i + 1) * iFact[i + 1] % mod;
  }
}

i32 comb (i32 n, i32 k) {
  if (!(0 <= k && k <= n)) return 0;
  return (i64) fact[n] * iFact[k] % mod * iFact[n - k] % mod;
}

void run (void) {
  i32 n, x;
  scanf ("%" SCNi32 "%" SCNi32, &n, &x);
  init_fact (n);
  i64 ans = 0;
  for (i32 i = 0; i < x; ++i) {
    for (i32 j = 0; 2 * j <= i; ++j) {
      i32 k = i - 2 * j;
      if (j + k > n) continue;
      i64 way = (i64) comb (n, j) * comb (n - j, k) % mod;
      ans += way;
    }
  }
  for (i32 i = x + 1; i < 2 * x; i += 2) {
    for (i32 j = i / 2; j >= 0; --j) {
      i32 k = i - 2 * j;
      if (j + k > n) break;
      i32 y = (i - x) / 2;
      i64 way = (i64) comb (n, j + k) * comb (j + k - 2 * y - 2, k) % mod;
      ans += way;
    }
  }
  if (x % 2 == 1) {
    for (i32 i = 2 * x; i <= 2 * n; i += 2) {
      ans += comb (n, i / 2);
    }
  }
  printf ("%" PRIi64 "\n", ans % mod);
}

int main (void) {
  run();
  return 0;
}

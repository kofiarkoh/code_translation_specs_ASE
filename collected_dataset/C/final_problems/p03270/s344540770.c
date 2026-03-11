#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

const i32 mod = 998244353;

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

i32 inv (i32 a) {
  return mod_pow (a, mod - 2);
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
  i32 k, n;
  scanf ("%" SCNi32 "%" SCNi32, &k, &n);
  init_fact (n + k);
  for (i32 i = 2; i <= 2 * k; ++i) {
    i32 c = 0;
    for (i32 j = 1, l = i - 1; j <= l; ++j, --l) {
      c += j <= k && l <= k ? 1 : 0;
    }
    i32 ans = 0;
    for (i32 j = 0, p = 1; j <= c; ++j, p = 2 * p % mod) {
      i32 way = (i64) comb (c, j) * p % mod;
      ans = (ans + (i64) way * comb (n - j + (k - (2 * c - j)) - 1, n - j)) % mod;
    }
    printf ("%" PRIi32 "\n", ans);
  }
}

int main (void) {
  run();
  return 0;
}

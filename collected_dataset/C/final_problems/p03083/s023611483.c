#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

typedef int32_t i32;
typedef int64_t i64;

const i32 mod = 1000000007;

i32 inv (i32 a) {
  i32 t = 1;
  while (a > 1) {
    t = (i64) t * (mod - mod / a) % mod;
    a = mod % a;
  }
  return t;
}

i32 modPow(i32 r, i32 n) {
  i32 t = 1;
  i32 s = r;
  while (n > 0) {
    if (n&1) t = (i64) t * s % mod;
    s = (i64) s * s % mod;
    n >>= 1;
  }
  return t;
}

i32 *fact = NULL;
i32 *iFact = NULL;
void init(const i32 n) {
  fact = (i32 *) calloc (n + 1, sizeof (i32));
  fact[0] = 1;
  for (i32 i = 1; i <= n; ++i) {
    fact[i] = (i64) i * fact[i - 1] % mod;
  }
  iFact = (i32 *) calloc (n + 1, sizeof (i32));
  iFact[n] = inv(fact[n]);
  for (i32 i = n - 1; i >= 0; --i) {
    iFact[i] = (i64) iFact[i + 1] * (i + 1) % mod;
  }
}

i32 comb (i32 n, i32 k) {
  if (!(0 <= k && k <= n)) return 0;
  return (i64) fact[n] * iFact[k] % mod * iFact[n - k] % mod;
}

void run (void) {
  i32 b, w;
  scanf ("%" SCNi32 "%" SCNi32, &b, &w);
  init(b + w);
  i32 l = 0;
  i32 r = 0;
  const i32 p = inv(2);
  for (i32 i = 1; i <= b + w; ++i) {
    printf ("%" PRIi64 "\n", (((i64)2 * mod + 1 - r - l) * p + l) % mod);
    if (i >= w) {
      l = (l + (i64)comb (i - 1, w - 1) * modPow (p, i)) % mod;
    }
    if (i >= b) {
      r = (r + (i64)comb (i - 1, b - 1) * modPow (p, i)) % mod;
    }
  }
}

int main (void) {
  run ();
  return 0;
}

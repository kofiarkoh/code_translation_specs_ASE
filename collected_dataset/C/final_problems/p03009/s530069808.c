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

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

void run (void) {
  i32 n, h, d;
  scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &n, &h, &d);
  init_fact (n);
  i64 p = 0;
  for (i32 i = 1; i <= n; ++i) {
    p += fact[i];
  }
  p %= mod;
  i32 *dp = ALLOC (h + d, i32) + d;
  dp[0] = fact[n];
  i64 sum = dp[0];
  for (i32 i = 1; i < h; ++i) {
    dp[i] = sum * p % mod;
    sum = (sum + dp[i] + mod - dp[i - d]) % mod;
  }
  printf ("%" PRIi64 "\n", sum);
}

int main (void) {
  run();
  return 0;
}

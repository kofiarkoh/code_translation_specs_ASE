#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

static void print_int(i64 n){if(n<0){putchar('-');n=-n;}if(n==0){putchar('0');return;}int s[20],len=0;while(n>0){s[len++]=n%10+'0';n/=10;}while(len>0){putchar(s[--len]);}}

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

i32 comb (i32 n, i32 k) {
  if (!(0 <= k && k <= n)) return 0;
  return (i64) fact[n] * iFact[k] % mod * iFact[n - k] % mod;
}

void run (void) {
  i32 b, w;
  scanf ("%" SCNi32 "%" SCNi32, &b, &w);
  init_fact (b + w);
  const i32 q = inv (2);
  const i64 buf = (i64) mod * mod;
  i64 ans = q;
  for (i32 i = 1, qq = (i64) q * q % mod; i <= b + w; ++i, qq = (i64) qq * q % mod) {
    print_int (ans);
    putchar ('\n');
    if (i >= b) {
      ans = (ans + buf - (i64) comb (i - 1, b - 1) * qq) % mod;
    }
    if (i >= w) {
      ans = (ans + (i64) comb (i - 1, w - 1) * qq) % mod;
    }
  }
}

int main (void) {
  run();
  return 0;
}

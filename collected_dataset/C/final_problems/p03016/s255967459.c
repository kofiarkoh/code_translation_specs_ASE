#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef uint64_t u64;

#define MIN(a,b) ((a)<(b)?(a):(b))

u64 mod = 0;

u64 digit (u64 n) {
  u64 p = 1;
  while (n > 0) {
    p = 10 * p;
    n /= 10;
  }
  return p;
}

const u64 f = 3;

void matmul (u64 *c, u64 *a, u64 *b) {
  u64 tmp[9] = {0,0,0,0,0,0,0,0,0};
  for (u64 i = 0; i < f; ++i) {
    for (u64 k = 0; k < f; ++k) {
      for (u64 j = 0; j < f; ++j) {
        tmp[i * f + j] += a[i * f + k] * b[k * f + j];
      }
    }
  }
  for (u64 i = 0; i < f * f; ++i) {
    c[i] = tmp[i] % mod;
  }
}

//c <- a^n
void mat_pow (u64 *c, u64 *a, u64 n) {
  u64 t[9] = {1,0,0,0,1,0,0,0,1};
  u64 s[9];
  for (u64 i = 0; i < f * f; ++i) {
    s[i] = a[i];
  }
  while (n > 0) {
    if (n & 1) matmul (t, t, s);
    matmul (s, s, s);
    n >>= 1;
  }
  for (u64 i = 0; i < f * f; ++i) {
    c[i] = t[i];
  }
}

void run (void) {
  u64 l, a, b;
  scanf ("%" SCNu64 "%" SCNu64 "%" SCNu64 "%" SCNu64, &l, &a, &b, &mod);
  u64 ans[9] = {1,0,0,0,1,0,0,0,1};
  u64 buf = 0;
  while (buf < l) {
    u64 p = digit (a + buf * b);
    u64 n = MIN(l - buf, (p - 1 - (a + buf * b)) / b + 1);
    buf += n;
    u64 mat[9] = {1,1,0,
                  0,1,0,
                  1,0,p%mod};
    mat_pow (mat, mat, n);
    matmul (ans, mat, ans);
  }
  printf ("%" PRIu64 "\n", (ans[6] * (a % mod) + ans[7] * (b % mod)) % mod);
}

int main (void) {
  run();
  return 0;
}

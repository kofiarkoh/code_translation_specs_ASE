#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>
#include<math.h>

typedef int32_t i32;
typedef int64_t i64;

#define MAX(a,b) ((a) > (b) ? (a) : (b))
#define MIN(a,b) ((a) < (b) ? (a) : (b))
#define ABS(a) ((a) > (0) ? (a) : -(a))

const i32 mod = 1000000007;

int cmp (const void *a, const void *b) {
  i32 d = *(i32 *)a - *(i32 *)b;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

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

#define POS(i,j) ((i) * (n + 1) + (j))

i32 calc (i32 x, i32 k, i32 *dp, i32 *a, i32 n) {
  //printf ("call (%" PRIi32 ",%" PRIi32 ")\n", x, k);
  if (k == 0){
    //printf ("x = %" PRIi32 "\n", x);
    return x;
  }
  if (x < a[0]) {
    return (i64) x * fact[k] % mod;
  }
  if (dp[POS(x,k)] >= 0){
    return dp[POS(x,k)];
  }
  i32 local = 0;
  i32 mul = 1;
  i32 i;
  for (i = k; i > 0 && a[i - 1] > x; --i) {
    mul = (i64) mul * i % mod;
  }
  i32 m = i;
  for (i = 0; i < m; ++i) {
    i32 c = calc (x % a[i], i, dp, a, n);
    i32 perm = (i64)fact[m - 1] * iFact[i] % mod;
    local = (local + (i64) c * perm) % mod;
  }
  //printf ("(%" PRIi32 ",%" PRIi32 ") = %" PRIi32 "\n", x, k, local);
  dp[POS(x,k)] = (i64)local * mul % mod;
  return local;
}

void run (void) {
  i32 n, x;
  scanf ("%" SCNi32 "%" SCNi32, &n, &x);
  i32 *a = (i32 *) calloc (n, sizeof (i32));
  for (i32 i = 0; i < n; ++i) {
    scanf ("%" SCNi32, a + i);
  }
  qsort (a, n, sizeof (i32), cmp);
  init(n);
  i32 *dp = (i32 *) calloc ((x + 1) * (n + 1), sizeof (i32));
  for (i32 i = 0; i < (x + 1) * (n + 1); ++i) {
    dp[i] = -1;
  }
  i32 ans = calc (x, n, dp, a, n);
  printf ("%" PRIi32 "\n", ans);
}

int main (void) {
  run ();
  return 0;
}

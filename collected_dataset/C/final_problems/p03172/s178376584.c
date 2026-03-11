#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

#define MIN(a,b) ((a)<(b)?(a):(b))
#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

#define POS(i, j) (((i) & 1) * (k + 2) + MIN((j), k + 1))

void run (void) {
  i32 n, k;
  scanf ("%" SCNi32 "%" SCNi32, &n, &k);
  i64 *dp = ALLOC (2 * (k + 1 + 1), i64);
  dp[POS(0, k)] = 1;
  const i32 mod = 1000000007;
  for (i32 i = 1; i <= n; ++i) {
    i32 a;
    scanf ("%" SCNi32, &a);
    for (i32 j = k; j >= 0; --j) {
      dp[POS(i, j)] = (dp[POS(i - 1, j)] + dp[POS(i, j + 1)] + mod - dp[POS(i - 1, j + a + 1)]) % mod;
    }
  }
  printf ("%" PRIi64 "\n", dp[POS(n, 0)]);
}

int main (void) {
  run();
  return 0;
}

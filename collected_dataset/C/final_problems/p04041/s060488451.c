#include <stdio.h>
#include <string.h>
#define BMAX 16
#define MOD 1000000007

int main(void)
{
  int i, j, k, n, x, y, z, xyz, mask, idx = 0, dp[2][1<<BMAX] = {0};
  long res = 1;
  scanf("%d %d %d %d", &n, &x, &y, &z);
  xyz = (1 << (x + y + z - 1)) | (1 << (y + z - 1)) | (1 << (z - 1));
  mask = ((1 << (x + y + z)) - 1) >> 1;
  dp[idx][0] = 1;
  for (i = 0; i < n; i++) {
    memset(dp[idx^1], 0, sizeof(dp[idx^1]));
    for (j = 0; j <= mask; j++) {
      if (dp[idx][j] > 0) {
        for (k = 0; k < 10; k++) {
          int used = (j << (k + 1)) | (1 << k);
          if ((used & xyz) != xyz) dp[idx^1][used&mask] = (dp[idx^1][used&mask] + dp[idx][j]) % MOD;
        }
      }
    }
    idx ^= 1;
  }
  for (i = 0; i < n; i++) res = (res * 10) % MOD;
  for (i = 0; i <= mask; i++) res = (res - dp[idx][i] + MOD) % MOD;
  printf("%ld\n", res);
}
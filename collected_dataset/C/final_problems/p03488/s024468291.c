#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include <limits.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))
#define Min(a, b) ((a) > (b) ? (b) : (a))
#define abs(x) ((x) > 0 ? (x) : -(x))
#define MOD (ll)1000000007 //10^9 + 7
#define endl printf("\n")
typedef long long ll;
#define MAX_N (1 << 17)

bool judge(int *a, int n, int x, bool dp[5010][9010]) {
  dp[0][0] = true;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= x; j++) {
      dp[i + 1][j] = dp[i][j];
      if (j - a[i] >= 0) dp[i + 1][j] |= dp[i][j - a[i]];
    }
  }
  return dp[n][x];
}

bool dp1[5010][9010], dp2[5010][9010];

int
main(int argc, char *argv[])
{
  int a[9010], x, y;
  char s[9010];

  scanf("%s", s);
  scanf("%d %d", &x, &y);

  int k = 0, l = 0;
  int n = strlen(s);
  int d[8010];
  for (int i = 0; i < n; i++) {
    if (s[i] == 'T') d[l++] = i;
  }
  int f = -1;
  for (int i = 0; i < l; i++) {
    a[i] = d[i] - f - 1;
    f = d[i];
  }
  a[l] = n - f - 1;
  l++;
  
  int b[9000], c[9000], e = 0, o = 0;
  int sum1 = 0, sum2 = 0;
  for (int i = 0; i < l; i++) {
    if (i % 2 == 0) {
      sum1 += (b[e] = a[i]);
      e++;
    } else {
      sum2 += (c[o] = a[i]);
      o++;
    }
  }

  int p = sum1 - x, q = sum2 - y;
  if (p % 2 == 0 && q % 2 == 0 && p >= 0 && q >= 0) {
    if (judge(b, e, p / 2, dp1) && judge(c, o, q / 2, dp2)) {
      printf("Yes\n");
      return 0;
    }
  }

  printf("No\n");

  

  return 0;
}

#include <stdio.h>
//#include <stdlib.h>
//#include <string.h>
//#include <stdbool.h>
#include <limits.h>
//#include <math.h>

#define Max(a, b) ((a>b)?a:b)

int main() {
  int n;
  scanf("%d", &n);
  int a[n];
  for (int i=0; i<n; i++)
    scanf("%d", a+i);
  long dp[n][2];
  dp[0][0] = 0;
  dp[0][1] = a[0];
  dp[1][0] = 0;
  dp[1][1] = Max(dp[0][1], a[1]);
  for (int i=2; i<n; i++) {
    if (i%2) {
      dp[i][0] = Max(dp[i-1][0], dp[i-2][0]+a[i]);
      dp[i][1] = Max(dp[i-1][1], dp[i-2][1]+a[i]);
    } else {
      dp[i][0] = Max(dp[i-1][1], dp[i-2][0]+a[i]);
      dp[i][1] = dp[i-2][1]+a[i];
    }
  }
  printf("%ld\n", dp[n-1][(n+1)%2]);
  return 0;
}

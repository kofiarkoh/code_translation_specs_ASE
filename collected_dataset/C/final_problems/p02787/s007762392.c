#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
int acs(const void *a, const void *b) { 
  return *(int*)a - *(int*)b;
} /* 1,2,3,4.. */
int des(const void *a, const void *b) {
  return *(int*)b - *(int*)a;
} /* 8,7,6,5.. */
int cmp_char(const void *a, const void *b) { 
  return *(char*)a - *(char*)b;
} /* a,b,c,d.. */
int cmp_str(const void *a, const void *b) {
  return strcmp(*(const char **)a, *(const char **)b);
} /* aaa,aab.. */
#define min(a,b) (a < b ? a : b)
#define max(a,b) (a > b ? a : b)
#define rep(i, l, r) for (int i = l; i < r; i++)
#define MAX 100005
#define INF 1000000007

typedef long long int lli;

int main(void) {
  int h, n;
  scanf("%d %d", &h, &n);
  int a[MAX], b[MAX];
  rep(i, 0, n) scanf("%d %d", &a[i], &b[i]);

  // dp[i]:体力iのモンスターを倒すのにかかる魔力の最小値
  int dp[MAX] = {0};
  rep(i, 0, MAX) dp[i] = INF;
  dp[0] = 0;
  rep(i, 0, MAX) {
    rep(ni, 0, n) {
      if (dp[i] == INF) continue;
      dp[i + a[ni]] = min(dp[i] + b[ni], dp[i + a[ni]]);
    }
  }

  int ans = INF;
  rep(i, h, MAX) {
    ans = min(ans, dp[i]);
  }
  printf("%d\n", ans);
  return 0;
}

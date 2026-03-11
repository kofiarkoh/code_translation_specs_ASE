#include<stdio.h>
#include<string.h>
#include<limits.h>
#define Size 15
long int dist[Size][Size], dp[1 << Size][Size];
int rec(int S, int v, int size);
int main(void)
{
  int i, j, n, e, s, t, d;
  scanf("%d%d\n", &n, &e);
  for (i = 0; i < n; i++) {
    for (j = 0; j < n; j++) {
      dist[i][j] = INT_MAX;
    }
  }
  memset(dp, -1, sizeof(dp));
  for (i = 0; i < e; i++) {
    scanf("%d%d%d", &s, &t, &d);
    dist[s][t] = d;
  }
  d = rec(0, 0, n);
  if (d == INT_MAX) printf("-1\n");
  else printf("%d\n", d);
}
int rec(int S, int v, int size)
{
  long int u, res = INT_MAX;
  if (dp[S][v] >= 0) return dp[S][v];
  if (S == (1 << size) - 1 && v == 0) return dp[S][v] = 0;
  for (u = 0; u < size; u++) {
    if (!(S >> u & 1)) {
      long int tmp = rec(S | 1 << u, u, size) + dist[v][u];
      if (res > tmp) res = tmp;
    }
  }
  return dp[S][v] = res;
}

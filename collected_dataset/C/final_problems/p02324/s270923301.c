#include<stdio.h>
#include<limits.h>
#define VMAX 15
long int dist[VMAX][VMAX], dp[1<<VMAX];
int evnum[VMAX], odd[VMAX];
int cpp(int size);
int main(void)
{
  int i, j, v, e, s, t, d, sum = 0;
  for (i = 0; i < (1 << VMAX); i++) dp[i] =INT_MAX;
  for (i = 0; i < VMAX; i++) {
    for (j = 0; j < VMAX; j++) dist[i][j] = INT_MAX;
  }
  dp[0] = 0;
  scanf("%d%d", &v, &e);
  for (i = 0; i < e; i++) {
    scanf("%d%d%d", &s, &t, &d);
    if (d < dist[s][t]) {
      dist[s][t] = d;
      dist[t][s] = d;
    }
    sum += d;
    evnum[s]++;
    evnum[t]++;
  }
  printf("%d\n", sum + cpp(v));
}
int cpp(int size)
{
  int i, j, k, S, count = 0;
  long long int limit;
  for (i = 0; i < size; i++) {
    if (evnum[i] & 1) odd[count++] = i;
  }
  for (k = 0; k < size; k++) {
    for (i = 0; i < size; i++) {
      for (j = 0; j < size; j++) {
        long int newlen = dist[i][k] + dist[k][j];
        if (newlen < dist[i][j]) dist[i][j] = newlen;
      }
    }
  }
  limit = (1 << count) - 1;
  for (S = 0; S <= limit; S++) {
    for (i = 0; i < count; i++) {
      if (!(S & (1 << i))) {
        for (j = i + 1; j < count; j++) {
          if (!(S & (1 << j))) {
            long int tmp = dp[S] + dist[odd[i]][odd[j]], s = S | (1 << i) | (1 << j);
            if (tmp < dp[s]) dp[s] = tmp;
          }
        }
      }
    }
  }
  return dp[limit];
}

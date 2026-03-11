#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
#define NMAX 1000
#define MIN(a,b) ((a) < (b) ? (a) : (b))
int cmatrix[NMAX][NMAX], dmatrix[NMAX][NMAX], visit[NMAX], dist[NMAX], prev[NMAX], h[NMAX];
int Maxflow(int, int, int);
int Mincostflow(int, int, int, int);
int main(void)
{
  int i, v, e, f, a, b, c, d, mincost = 0;
  scanf("%d %d %d", &v, &e, &f);
  for (i = 0; i < e; i++) {
    scanf("%d %d %d %d", &a, &b, &c, &d);
    cmatrix[a][b] = c;
    cmatrix[b][a] = 0;
    dmatrix[a][b] = d;
    dmatrix[b][a] = -d;
  }
  printf("%d\n", Mincostflow(0, v - 1, v, f));
}
int Maxflow(int s, int t, int size)
{
  int i;
  for (i = 0; i < size; i++) {
    dist[i] = INT_MAX;
    prev[i] = -1;
    visit[i] = 0;
  }
  dist[s] = 0;
  while(1) {
    int u = -1, min = INT_MAX;
    for (i = 0; i < size; i++) {
      if (!visit[i] && dist[i] < min) {
        u = i;
        min = dist[i];
      }
    }
    if (u == -1) break;
    visit[u] = 1;
    for (i = 0; i < size; i++) {
      if (cmatrix[u][i] > 0) {
        int newlen = dist[u] + dmatrix[u][i] + h[u] - h[i];
        if (newlen < dist[i]) {
          dist[i] = newlen;
          prev[i] = u;
        }
      }
    }
  }
  return dist[t] != INT_MAX;
}
int Mincostflow(int s, int t, int size, int f)
{
  int mincost = 0;
  while (f > 0 && Maxflow(s, t, size)) {
    int i, delta = f;
    for (i = t; i != s; i = prev[i]) {
      delta = MIN(delta, cmatrix[prev[i]][i]);
    }
    f -= delta;
    for (i = t; i != s; i = prev[i]) {
      cmatrix[prev[i]][i] -= delta;
      cmatrix[i][prev[i]] += delta;
    }
    for (i = 0; i < size; i++) h[i] += dist[i];
    mincost += delta * h[t];
  }
  if (f) mincost = -1;
  return mincost;
}

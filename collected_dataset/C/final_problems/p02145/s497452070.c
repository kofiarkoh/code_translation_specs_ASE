#include<stdio.h>
#include<string.h>
#include<limits.h>
#define CMAX 26
#define SMAX 100
#define MIN(a,b) ((a) < (b) ? (a) : (b))

int net[CMAX][CMAX], matrix[CMAX+2][CMAX+2], iter[CMAX+2], level[CMAX+2];
int Bfs(int, int, int);
int Dfs(int, int, int, int);
int main(void)
{
  int i, j, k, n, v;
  char buf[SMAX];
  scanf("%d", &n);
  for (i = 0; i < n; i++) {
    int len, a, b;
    scanf("%s%*c", buf);
    len = strlen(buf);
    a = buf[0] - 'a';
    b = buf[len-1] - 'a';
    net[a][b]++;
  }
  for (i = 0; i < CMAX; i++) {
    int maxflow = 0, inflow = 0, outflow = 0;
    for (j = 0; j < CMAX; j++) {
      inflow += net[j][i];
      outflow += net[i][j];
    }
    v = CMAX + 2;
    memset(matrix, 0, sizeof(matrix));
    for (j = 0; j < CMAX; j++) {
      for (k = 0; k < CMAX; k++) {
        if (j == i && k == i) matrix[CMAX][CMAX+1] = net[i][i];
        else if (j == i && k != i) matrix[CMAX][k] = net[i][k];
        else if (j != i && k == i) matrix[j][CMAX+1] = net[j][i];
        else matrix[j][k] = net[j][k];
      }
    }
    while (Bfs(CMAX, CMAX + 1, v)) {
      memset(iter, 0, sizeof(iter));
      maxflow += Dfs(CMAX, CMAX + 1, v, INT_MAX);
    }
    if (outflow == maxflow && inflow > 0) printf("%c\n", i + 'a');
  }
}

int Bfs(int s, int t, int size)
{
  int i, v, head = 0, tail = 0, queue[CMAX+2];
  memset(level, -1, sizeof(level));
  level[s] = 0;
  queue[tail++] = s;
  do {
    i = queue[head++];
    for (v = 0; v < size; v++) {
      if (level[v] < 0 && matrix[i][v] > 0) {
        queue[tail++] = v;
        level[v] = level[i] + 1;
      }
    }
  } while (head != tail);
  return level[t] >= 0;
}

int Dfs(int u, int t, int size, int f)
{
  int *i, fsum = 0;
  if (u == t) return f;
  for (i = &iter[u]; *i < size; (*i)++) {
    if (level[u] < level[(*i)] && matrix[u][(*i)] > 0) {
      int ftemp = Dfs((*i), t, size, MIN(f - fsum, matrix[u][(*i)]));
      if (ftemp > 0) {
        matrix[u][(*i)] -= ftemp;
        matrix[(*i)][u] += ftemp;
        fsum += ftemp;
        if (fsum == f) break;
      }
    }
  }
  return fsum;
}

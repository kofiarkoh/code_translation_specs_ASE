#include <stdio.h>

#define N 101
#define WHITE 0
#define GRAY 1
#define BLACK 2

int n, time, data[N][N], color[N], d[N], f[N]; 

void dfs();
void visit(int);

int main() {
  int i, j, k, l, u, v;
  
  scanf("%d", &n);

  for (i = 0; i < N; i++) {
    for (j = 0; j < N; j++) {
      data[i][j] = 0;
    }
  }

  for (i = 0; i < n; i++) {
    scanf("%d%d", &u, &k);
    
    for (j = 0; j < k; j++) {
      scanf("%d", &l);
      data[u][l] = 1;
    }
  }
  
  dfs();

  for (i = 1; i <= n; i++) {
    printf("%d %d %d\n", i, d[i], f[i]);
  }


  return 0;
}


void dfs() {
  int i;

  for (i = 1; i <= n; i++) {
    color[i] = WHITE;
  }
  time = 0;
  for (i = 1; i <= n; i++) {
    if (color[i] == WHITE) visit(i);
  }

}


void visit(int u) {
  int i;
  
  color[u] = GRAY;
  d[u] = ++time;
  
  for (i = 1; i <= n; i++) {
    if (data[u][i] == 0) continue;
    if (color[i] == WHITE) {
      visit(i);
    }
  }
  color[u] = BLACK;
  f[u] = ++time;
}


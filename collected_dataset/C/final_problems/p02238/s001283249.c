#include<stdio.h>
#define N 101
#define WHITE -1
#define GRAY 0
#define BRACK 1
int G[N][N] = {}, n, time;
int d[N] = {}, f[N] = {}, color[N];

void DFSvisit(int u){
  int i;
  color[u] = GRAY;
  d[u] = ++time;
  for(i = 1; i <= n; i++){
      if(G[u][i] == 1){
        if(color[i] == WHITE) DFSvisit(i);
      }
  }
  color[u] = BRACK;
  f[u] = ++time;
}

void DFS(void){
  int i, j;
  for(i = 1; i <= n; i++){
        color[i] = WHITE;
        time = 0;
  }
  for(i = 1; i <= n; i++){
        if(color[i] == WHITE) DFSvisit(i);
  }
}

int main(){
  int u, k, v, i, j;
  scanf("%d", &n);
  for(i = 0; i < n; i++){
    scanf("%d%d", &u, &k);
    for(j = 0; j < k; j++){
      scanf("%d", &v);
      G[u][v] = 1;
    }
  }
  DFS();
  for(i = 1; i <= n; i++){
    printf("%d %d %d\n", i, d[i], f[i]);
  }
  return 0;
}
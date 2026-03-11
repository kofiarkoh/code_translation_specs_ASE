#include<stdio.h>

int visit[100] = {0};
int graph[100][100] = {0};
int n, m, route;

void search(int lavel, int goal) {
  visit[lavel] = 1;
  if ( lavel == goal ) {
    route = 1;
  }
  else {
    int to;
    for ( to = 0; to < n; to++ ) {
      if ( !visit[to] && graph[lavel][to] ) {
        search(to, goal);
      }
    }
  }
}

int main(void) {
  int path[100][100];
  int a, b;
  int i, j, k, l;
  int ct;
  scanf("%d %d", &n, &m);
  for ( i = 0; i < m; i++ ) {
    scanf("%d %d", &a, &b);
    path[i][0] = a-1;
    path[i][1] = b-1;
    graph[a-1][b-1] = graph[b-1][a-1] = 1;
  }
  for ( i = 0; i < m; i++ ) {
    ct = 0;
    graph[path[i][0]][path[i][1]] = 0;
    graph[path[i][1]][path[i][0]] = 0;
    for ( j = n-1; j >= 0; j-- ) {
      for ( k = 0; k < j; k++ ) {
        for ( l = 0; l < n; l++ ) {
          visit[l] = 0;
        }
        route = 0;
        search(k, j);
        if ( !route ) {
          ct += 1;
        }
      }
    }
    printf("%d\n", ct);
  }
  
  return 0;
}
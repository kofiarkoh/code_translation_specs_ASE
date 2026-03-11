#include<stdio.h>
#include<stdlib.h>
#include<limits.h>

int **dist;

void Floyd(int);

int main(void)
{
  int i, j, v, e, s, t, d;
  scanf("%d %d", &v, &e);
  dist = (int**)malloc(sizeof(int*) * v);
  for (i = 0; i < v; i++) {
    dist[i] = (int*)malloc(sizeof(int) * v);
    for (j = 0; j < v; j++) {
      dist[i][j] = ((i == j) ? 0 : INT_MAX);
    }
  }
  for (i = 0; i < e; i++) {
    scanf("%d %d %d", &s, &t, &d);
    dist[s][t] = d;
  }
  Floyd(v);
  for (i = 0; i < v; i++) {
    if (dist[i][i] < 0) {
      printf("NEGATIVE CYCLE\n");
      return 0;
    }
  }
  for (i = 0; i < v; i++) {
    for (j = 0; j < v - 1; j++) {
      if (dist[i][j] == INT_MAX) {
        printf("INF ");
      } else {
        printf("%d ", dist[i][j]);
      }
    }
    if (dist[i][j] == INT_MAX) {
      printf("INF\n");
    } else {
      printf("%d\n", dist[i][j]);
    }
    free(dist[i]);
  }
  free(dist);
  return 0;
}

void Floyd(int size)
{
  int k, i, j;
  for (k = 0; k < size; k++) {
    for (i = 0; i < size; i++) {
      if (dist[i][k] == INT_MAX) continue;
      for (j = 0; j < size; j++) {
        if (dist[k][j] == INT_MAX) continue;
        if (dist[i][j] > dist[i][k] + dist[k][j]) {
          dist[i][j] = dist[i][k] + dist[k][j];
        }
      }
    }
  }
}

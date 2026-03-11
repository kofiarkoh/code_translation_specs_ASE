#include <stdio.h>

int r[100][100];
int n;

int dijkstra(int start, int goal) {
  int i;
  int minimum, p;
  int d[100], f[100];

  for(i = 0; i < 100; i++) {
    d[i] = 100000;
    f[i] = 0;
  }
  d[start-1] = 1;
  while(1) {
    minimum = 100000;
    for(i = 0; i < n; i++) {
      if(d[i] < minimum && f[i] == 0) {
	minimum = d[i];
	p = i;
      }
    }
    if(minimum == 100000) break;
    f[p] = 1;

    for(i = 0; i < n; i++) {
      if(d[p] + r[p][i] < d[i]) {
	d[i] = d[p] + r[p][i];
      }
    }
  }

  return d[goal-1];
}

int main() {
  int i, j, r1, k1, p, s1, d1, v1, temp;

  for(i = 0; i < 100; i++)
    for(j = 0; j < 100; j++)
      r[i][j] = 100000;

  scanf("%d", &n);
  for(i = 0; i < n; i++) {
    scanf("%d %d", &r1, &k1);
    for(j = 0; j < k1; j++) {
      scanf("%d", &temp);
      r[r1-1][temp-1] = 1;
    }
  }
  scanf("%d", &p);
  for(i = 0; i < p; i++) {
    scanf("%d %d %d", &s1, &d1, &v1);
    temp = dijkstra(s1, d1);
    if(v1 - temp < 0) {
      printf("NA\n");
    } else {
      printf("%d\n", temp);
    }
  }
  return 0;
}
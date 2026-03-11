#include <stdio.h>
//#include <stdlib.h>
//#include <string.h>
//#include <stdbool.h>
//#include <limits.h>
//#include <math.h>

#define Min(a, b) (a < b ? a : b)

int main() {
  int h, w;
  scanf("%d%d", &h, &w);
  int c[10][10], a[h][w];
  for (int i=0; i<10; i++)
    for (int j=0; j<10; j++)
      scanf("%d", c[i]+j);
  for (int i=0; i<h; i++)
    for (int j=0; j<w; j++)
      scanf("%d", a[i]+j);
  int cost[10];
  for (int i=0; i<10; i++)
    cost[i] = c[i][1];
  for (int i=0; i<10; i++) {
    for (int j=0; j<10; j++) {
      for (int k=0; k<10; k++) {
        cost[j] = Min(cost[j], cost[k]+c[j][k]);
      }
    }
  }
  int ans = 0;
  for (int i=0; i<h; i++) {
    for (int j=0; j<w; j++) {
      if (a[i][j] == -1)
        continue;
      else
        ans += cost[a[i][j]];
    }
  }
  printf("%d\n", ans);
  return 0;
}

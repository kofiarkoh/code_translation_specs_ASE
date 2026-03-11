#include<stdio.h>
#include<stdlib.h>
struct goods {
  int profit;
  int weight;
};
int **C;
struct goods *item;
int knapsack(int size, int limit);
int main(void)
{
  int i, n, w;
  scanf("%d%d", &n, &w);
  C = (int**)malloc(sizeof(int*) * (n + 1));
  item = (struct goods*)malloc(sizeof(struct goods) * (n + 1));
  for (i = 0; i <= n; i++) {
    C[i] = (int*)malloc(sizeof(int) * (w + 1));
    C[i][0] = 0;
    if (i) scanf("%d %d", &item[i].profit, &item[i].weight);
  }
  for (i = 0; i <= w; i++) C[0][i] = 0;
  printf("%d\n", knapsack(n, w));
  for (i = 0; i <= n; i++) free(C[i]);
  free(C);
  free(item);
}
int knapsack(int size, int limit)
{
  int i, j;
  for (i = 1; i <= size; i++) {
    for (j = 1; j <= limit; j++) {
      C[i][j] = C[i-1][j];
      if (item[i].weight > j) continue;
      if (item[i].profit + C[i-1][j-item[i].weight] > C[i-1][j]) {
        C[i][j] = item[i].profit + C[i-1][j-item[i].weight];
      }
    }
  }
  return C[size][limit];
}
/*#include<stdio.h>
#include<stdlib.h>
#include<limits.h>
struct knapsack {
  int profit;
  int weight;
};
struct knapsack *goods;
int maxprofit, n;
int cmp(const void *a, const void *b);
void backtrack(int phase, int p, int w);
int main (void)
{
  int i, limit;
  scanf("%d %d", &n, &limit);
  goods = (struct knapsack*)malloc(sizeof(struct knapsack) * (n + 1));
  for (i = 0; i < n; i++) scanf("%d %d", &goods[i].profit, &goods[i].weight);
  goods[n].profit = 0;
  goods[n].weight = INT_MAX;
  qsort(goods, n, sizeof(struct knapsack), cmp);
  backtrack(0, 0, limit);
  printf("%d\n", maxprofit);
  free(goods);
}
int cmp (const void *a, const void *b)
{
  struct knapsack ta = *(struct knapsack*)a, tb = *(struct knapsack*)b;
  if ((double)ta.profit / ta. weight < (double)tb.profit / tb.weight) return 1;
  else if ((double)ta.profit / ta. weight > (double)tb.profit / tb.weight) return -1;
  else return 0;
}
void backtrack(int phase, int p, int w)
{
  if (phase >= n) {
    if (p > maxprofit) maxprofit = p;
  } else {
    double z = 0.0, u = w;
    int i = phase + 1;
    if (goods[phase].weight <= w) {
      backtrack(phase + 1, p + goods[phase].profit, w - goods[phase].weight);
    }
    while (goods[i].weight <= u) {
      u -= goods[i].weight;
      z += goods[i].profit;
      i++;
    }
    z += u * goods[i].profit / goods[i].weight;
    if (z + p > maxprofit) {
      printf("maxprofit: %d, z + p: %f, u: %f\n", maxprofit, z + p, u);
      backtrack(phase + 1, p, w);
    }
  }
}
*/

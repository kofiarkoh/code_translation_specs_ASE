#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include <limits.h>
#define Max(a, b) ((a) > (b) ? (a) : (b))
#define Min(a, b) ((a) > (b) ? (b) : (a))
#define abs(x) ((x) > 0 ? (x) : -(x))
#define MOD (ll)1000000007 //10^9 + 7
#define endl printf("\n")
typedef long long ll;
#define MAX_N (1 << 17)

int ans = INT_MIN;
int d[55], n, c[55], tmp[55];

int asc(const void *a, const void *b) {
  return *(int *)a - *(int *)b;
}

void dfs(int i) {
  if (i == n) {
    c[n] = 0;
    for (int j = 0; j < n + 1; j++) tmp[j] = c[j];
    qsort(tmp, n + 1, sizeof(int), asc);
    
    int min = INT_MAX;
    for (int j = 0; j < n; j++)
      min = Min(min, Min(tmp[j + 1] - tmp[j], 24 + tmp[j] - tmp[j + 1]));
    
    min = Min(min, 24 + tmp[0] - tmp[n]);
    ans = Max(ans, min);
    return;
  }
  c[i] = d[i];
  dfs(i + 1);
  c[i] = - d[i];
  dfs(i + 1);

  return;
}

int
main(int argc, char *argv[])
{
  scanf("%d", &n);
  for (int i = 0; i < n; i++) scanf("%d", &d[i]);

  if (n >= 24) printf("0\n");
  else {
    dfs(0);
    printf("%d\n", ans);
  }

  return 0;
}

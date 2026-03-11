#include <stdio.h>

int main() {
  int n;
  scanf("%d", &n);
  int a[n][n-1], b[n], c[n], mcnt = 0, ans = 0;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n-1; j++) {
      scanf("%d", a[i]+j);
    }
  }
  for (int i = 0; i<n; i++) {
    b[i] = 0;
  }
  while (mcnt < n*(n-1)/2) {
    int noMatch = 0;
    for (int i=0; i<n; i++) {
      c[i] = 0;
    }
    for (int i=0; i<n; i++) {
      if (c[i] != 0) continue;
      int idx = a[i][b[i]]-1;
      if (c[idx] != 0) continue;
      if (i+1 == a[idx][b[idx]]) {
        mcnt++;
        b[i]++;
        b[idx]++;
        c[i]++;
        c[idx]++;
        noMatch++;
      }
    }
    if (noMatch == 0) {
      printf("-1\n");
      return 0;
    }
    ans++;
  }
  printf("%d\n", ans);
  return 0;
}

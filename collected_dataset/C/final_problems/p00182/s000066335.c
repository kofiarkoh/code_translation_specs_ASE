#include <stdio.h>
#include <stdlib.h>

int b[50]; // beaker status
int d[50]; // used status
int f[50]; // filled status

int compare(const void *a, const void *b) {
  return *(int *)b - *(int *)a;
}

int dcheck(int n) {
  int i, r;
  for (i = 0, r = 0; i < n; i++)
    r += d[i];
  if (r == n)
    return 1;
  else
    return 0;
}

int beaker(int n, int m) {
  int i, t;
  f[m] = 0;
  d[m] = 1;

  if (n == 1) return 1;
  for (i = m + 1, t = b[m]; i < n; i++) {
    if (t >= b[i] && f[i] == 0) {
      t -= b[i];
      d[i] = f[i] = 1;
    }
    if (t == 0)
      if (dcheck(n))
        return 1;
      else
        beaker(n, m + 1);
  }
  return 0;
}

void init() {
  int i;
  for (i = 0; i < 50; i++)
    b[i] = f[i] = d[i] = 0;
  return;
}

int main() {
  int i, n, m;

  for (;;) {
    scanf("%d", &n);
    if (n == 0)
      return 0;
    init();
    for (i = 0; i < n; i++)
      scanf("%d", &b[i]);
    qsort(b, n, sizeof(int), compare);
    m = 0; // set max value posion
    printf("%s", beaker(n, m) ? "YES\n" : "NO\n");
  }
  return -1;
}
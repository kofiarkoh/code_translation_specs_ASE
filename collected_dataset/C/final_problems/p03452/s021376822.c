#include <stdio.h>
#include <string.h>
#define NMAX 100000

int ufpar[NMAX], ufw[NMAX];

int UF_find(int);
int UF_weight(int);
int UF_union(int, int, int);
int UF_diff(int*, int, int);
int main(void)
{
  int i, n, m, l, r, d;
  scanf("%d %d", &n, &m);
  memset(ufpar, -1, sizeof(ufpar));
  for (i = 0; i < m; i++) {
    scanf("%d %d %d", &l, &r, &d);
    if (!UF_union(l - 1, r - 1, d)) {
      if (ufw[l-1] != ufw[r-1] + d) {
        printf("No\n");
        return 0;
      }
    }
  }
  printf("Yes\n");
}

int UF_find(int a)
{
  int p;
  if (ufpar[a] < 0) return a;
  p = UF_find(ufpar[a]);
  ufw[a] += ufw[ufpar[a]];
  return ufpar[a] = p;
}

int UF_weight(int a)
{
  UF_find(a);
  return ufw[a];
}

int UF_union(int x, int y, int w)
{
  w -= UF_weight(x);
  w += UF_weight(y);
  x = UF_find(x);
  y = UF_find(y);
  if (x == y) return 0;
  if (ufpar[x] > ufpar[y]) {
    ufpar[y] += ufpar[x];
    ufpar[x] = y;
    ufw[x] = w;
  } else {
    ufpar[x] += ufpar[y];
    ufpar[y] = x;
    ufw[y] = -w;
  }
  return 1;
}

int UF_diff(int *ans, int x, int y)
{
  if (UF_find(x) != UF_find(y)) return 0;
  else {
    *ans = ufw[x] - ufw[y];
    return 1;
  }
}
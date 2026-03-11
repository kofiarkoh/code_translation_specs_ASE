#include <stdio.h>

int v[200001] = { 0 };

void init(int n) {
  for(int i = 0;i < n;i++) v[i] = i;
}

int root(int x) {
  if(v[x] == x) {
    return x;
  } else {
    v[x] = root(v[x]);
    return v[x];
  }
}

void unite(int x, int y) {
  x = root(x);
  y = root(y);
  if(x == y) return;
  if(v[x] > v[y]) { int tmp = x; x = y; y = tmp; }
  v[y] = v[x];
}

int main(void) {
  int n, m;
  scanf("%d%d", &n, &m);
  init(n);
  for(int i = 0;i < m;i++) {
    int x, y;
    scanf("%d%d", &x, &y);
    x--, y--;
    unite(x, y);
  }
  for(int i = 0;i < n;i++) {
    v[i] = root(v[i]);
  }
  int r[n];
  for(int i =0;i < n;i++) {
    r[i] = 0;
  }
  for(int i = 0;i < n;i++) {
    r[v[i]]++;
  }
  int result = 0;
  for(int i = 0;i < n;i++) {
    if(result < r[i]) result = r[i];
  }
  printf("%d\n", result);
  return 0;
}

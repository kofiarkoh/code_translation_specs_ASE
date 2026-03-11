#include <stdio.h>
#include <stdlib.h>

#define rep(i, n) for (int i = 0; i < n; i = i + 1)
#define array(N, t) (t*)calloc(N, sizeof(t))
#define ll long long
#define INF ((1LL << 62) - (1LL << 31))
#define max(p, q)((p) > (q) ? (p) : (q))
#define min(p, q)((p) < (q) ? (p) : (q))
#define swap(type, a, b) { type temp = a; a = b; b = temp; }

int N;
int * data;

void init() {
  // child: parent's index
  // root: -size
  data = array(N, int);

  rep(i, N) {
    // at first, all data is root and size is -1
    data[i] = -1;
  }
}

int find(int x) {
  if (data[x] < 0) {
    return x;
  }

  return data[x] = find(data[x]);
}

int same(int x, int y) {
  return find(x) == find(y);
}

int unite(int x, int y) {
  x = find(x);
  y = find(y);
  if (x == y) {
    return 0;
  }
  // becasue data has minus size, reverse the inequality sign
  if (data[x] > data[y]) {
    swap(int, x, y);
  }
  data[x] += data[y];
  data[y] = x;
  return 1;
}

int size(int x) {
  return -data[find(x)];
}

int main() {
  int M;
  scanf("%d%d", &N, &M);

  init();

  rep(i, M) {
    int A, B;
    scanf("%d%d", &A, &B);
    A -= 1;
    B -= 1;
    unite(A, B);
  }

  int ans = 0;
  rep(i, N) {
    ans = max(ans, size(i));
  }

  printf("%d\n", ans);

  free(data);

  return 0;
}

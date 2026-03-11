#include <stdio.h>
#include <math.h>

int prime[10005];

void solve(void) {
  int N;
  int cnt = 0;
  int i, j;

  scanf("%d", &N);

  for (i = 0; i < N; i++) {
    int x;
    int f = 0;
    scanf("%d", &x);
    int root = sqrt((double)x);

    for (j = 2; j <= root && j != x; j++) {
      if (x % j == 0) {
        f = 1;
      }
    }
    cnt += !f;
  }

  printf("%d\n", cnt);
}

int main(void) {
  solve();

  return 0;
}
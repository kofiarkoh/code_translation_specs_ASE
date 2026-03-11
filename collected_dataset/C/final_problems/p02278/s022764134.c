#include <stdio.h>
#include <stdlib.h>

int n;

int search(int *A, int w);

int main(int argc, char *argv[])
{
  int i, j, k, l, t;
  int *A, *R, *g, tmp, minj;
  int cost = 0;

  scanf("%d", &n);
  A = (int *)malloc(sizeof(int) * n);
  R = (int *)malloc(sizeof(int) * n);
  g = (int *)malloc(sizeof(int) * n);
  for (i = 0; i < n; i++) {
    scanf("%d", &A[i]);
    R[i] = A[i];
    g[i] = 0;
  }

  for (i = 0; i < n; i++) {
    minj = i;
    for (j = i; j < n; j++) {
      if (R[minj] > R[j]) minj = j;
    }
    tmp = R[minj];
    R[minj] = R[i];
    R[i] = tmp;
  }

  for (i = 0; i < n; i++) {
    if (A[i] == R[i]) continue;

    minj = i;
    g[0] = i;
    k = 1;
    j = i;
    while (A[j] != R[i]) {
      j = search(R, A[j]);
      if (A[j] < A[minj]) minj = j;
      g[k++] = j;
    }
    l = search(g, minj);
    for (j = k; j > 1; j--) {
      t = (l + j - 1) % k;
      cost += A[g[t]] + A[g[t+1]];
      tmp = A[g[t+1]];
      A[g[t+1]] = A[g[t]];
      A[g[t]] = tmp;
    }
  }

  printf("%d\n", cost);

  return 0;
}

int search(int *A, int w)
{
  int i;
  int found;

  found = 0;
  for (i = 0; i < n; i++) {
    if (A[i] == w) {
      found = 1;
      break;
    }
  }

  if (found == 1) return i;
  else return -1;
}
#include <stdio.h>

int main()
{
  int i, k, n, q, b, e, t, u;
  fscanf(stdin, "%d", &n);
  int a[n];
  for (i=0; i<n; i++)
    fscanf(stdin, "%d", &a[i]);

  fscanf(stdin, "%d", &q);
  for (i=0; i<q; i++) {
    fscanf(stdin, "%d %d %d", &b, &e, &t);

    for (k=0; k<e-b; k++) {
      u = a[b+k];
      a[b+k] = a[t+k];
      a[t+k] = u;
    }
  }
  for (i=0; i<n-1; i++)
    printf("%d ", a[i]);
  printf("%d\n", a[i]);

  return 0;
}


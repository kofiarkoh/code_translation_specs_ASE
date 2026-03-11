#include <stdio.h>
#include <stdlib.h>
#include <strings.h>

typedef struct {
  int s;
  int t;
} A;

int compar(const void* a, const void* b)
{
  if (((A*)a)->s > ((A*)b)->s) {
    return 1;
  } else if (((A*)a)->s < ((A*)b)->s) {
    return -1;
  } else {
    if (((A*)a)->t > ((A*)b)->t) {
      return 1;
    } else if (((A*)a)->t < ((A*)b)->t) {
      return -1;
    }
  }
  return 0;
}

int main()
{
  int i, j, n, s, t, left, right, nans=0;

  fscanf(stdin, "%d", &n);
  A *a = (A*)malloc(sizeof(A)*n);
  for (i=0; i<n; i++)
    fscanf(stdin, "%d %d", &(a[i].s), &(a[i].t));

  qsort(a, n, sizeof(A), compar);

  left=-1;
  j=0;
  int ll=a[n-1].t;
  while (left<ll) {
    right = ll+1;
    for (i=j; i<n; i++) {
      s = a[i].s;
      t = a[i].t;
      if (s>right) break;
      if (s>left && t<right) {
        right = t;
        j=i;
      }
    }
    if (right== ll+1) break;
    left = right;
    nans++;
  }
  printf("%d\n", nans);
  return 0;
}


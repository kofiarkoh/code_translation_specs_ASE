#include <stdio.h>
#include <stdlib.h>

int main()
{
  int i, j, k, n, m;
  int *a, *b;
  fscanf(stdin, "%d\n", &n);
  a = (int*)malloc(sizeof(int)*n);
  for (i=0; i<n; i++)
    fscanf(stdin, "%d", &a[i]);

  fscanf(stdin, "%d\n", &m);
  b = (int*)malloc(sizeof(int)*m);
  for (i=0; i<m; i++)
    fscanf(stdin, "%d", &b[i]);
  j=0;
  for (i=0; i<m; i++) {
    for (; j<n && b[i]>=a[j]; j++) {
      if (b[i]==a[j]) goto next;
    }
    putc_unlocked('0', stdout);
    putc_unlocked('\n', stdout);
    break;
  next:
    ;
  }
  if (i==m) {
    putc_unlocked('1', stdout);
    putc_unlocked('\n', stdout);
  }
    
  return 0;
}


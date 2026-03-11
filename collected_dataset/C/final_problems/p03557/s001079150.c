#include <stdio.h>
#include <stdlib.h>

int asc(const void *a, const void *b){
  return *(int *)a - *(int *)b;
}

int
main(int argc, char *argv[])
{
  int n;
  scanf("%d", &n);
  int a[100000], b[100000], c[100000];
  for(int i = 0; i < n; i++) scanf("%d", &a[i]);
  for(int i = 0; i < n; i++) scanf("%d", &b[i]);
  for(int i = 0; i < n; i++) scanf("%d", &c[i]);

  qsort(a, n, sizeof(int), asc);
  qsort(b, n, sizeof(int), asc);
  qsort(c, n, sizeof(int), asc);

  int k = 0; long long ia[100000], xb[100000];
  xb[n] = 0;
  for(int i = 0; i < n; i++){
    while(a[i] >= b[k] && k < n) k++;
    ia[i] = k;
  }
  
  k = 0;
  for(int i = 0; i < n; i++){
    while(b[i] >= c[k] && k < n) k++;
    xb[i] = n - k;
  }

  k = n - 1;
  long long sb[100000] = {};
  sb[n - 1] = xb[n - 1];
  while(k > 0){
    k--;
    sb[k] = sb[k + 1] + xb[k];
  }

  long long ans = 0;
  for(int i = 0; i < n; i++){
    ans += sb[ia[i]];
  }
  
  printf("%lld\n", ans);

  return 0;
}
				  

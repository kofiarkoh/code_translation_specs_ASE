#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#define MAX(a,b) ((a)>(b)?(a):(b))
int cmp(const void *a, const void *b)
{
  return *(long long int*)a - *(long long int*)b;
}

int main()
{
  int n, i, n_half;
  long long int a[10000];
  long long int ans=0;
  long long int ans1, ans2;

  scanf("%d",&n);
  for(i=0;i<n;i++) {
    scanf("%lld",&a[i]);
  }
  qsort(a, n, sizeof(long long int), cmp);
  if(n%2==0) {
    n_half = n/2;
    for(i=0;i<n_half-1;i++) {
      ans = ans + 2*a[n-1-i] - 2*a[i];
    }
    ans = ans + a[n_half] - a[n_half-1];
  } else {
    n_half = n/2;
    for(i=0;i<n_half-1;i++) {
      ans = ans + 2*a[n-1-i] - 2*a[i];
    }
    ans1 = ans - 2*a[n_half-1] + a[n_half] +   a[n_half+1];
    ans2 = ans -   a[n_half-1] - a[n_half] + 2*a[n_half+1];
    ans = MAX(ans1, ans2);
  }
  printf("%lld",ans);
  //printf("\n%d",n_half);
  return 0;
}

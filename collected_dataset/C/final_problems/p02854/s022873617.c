#include <stdio.h>
#include <math.h>
#include <limits.h>
int main(void){
  int n, i, j;
  scanf("%d", &n);
  long long a[n];
  for(i=0; i<n; i++)
    scanf("%lld", &a[i]);

  long long sl[n], sr[n];

  for(i=0; i<n; i++){
    sl[i]=0; sr[i]=0;
  }
  
  long long ans = LLONG_MAX;

  for(i=1; i<n; i++){
    for(j=0; j<i; j++){
      sl[i] += a[j];
    }
    for(; j<n; j++){
      sr[i] += a[j];
    }
    long long t = sl[i]-sr[i];
    if(t<0) t*=-1;
    if(ans > t){
      ans = t;
    }
  }
  printf("%lld", ans);
  return 0;
}

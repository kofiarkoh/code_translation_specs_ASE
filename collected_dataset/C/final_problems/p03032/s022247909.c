#include <stdio.h>
#include <stdlib.h>
#define max(x, y) ((x) > (y) ? (x) : (y))
#define min(x, y) ((x) > (y) ? (y) : (x))

int dsc(const void *a, const void *b){
  long long *A = (long long *)a, *B = (long long *)b;
  if(*A > *B) return  1;
  if(*B > *A) return -1;
  return 0;
}

int
main(int argc, char *argv[])
{
  int n, k;
  long long v[50]; 
  scanf("%d%d", &n, &k);
  int j = 0;
  for(int i = 0; i < n; i++) scanf("%lld", &v[i]);

  long long ans = 0, vsum;

  int l = min(k / 2, n);
  for(int j = 0; j <= l; j++){
    vsum = 0;
    for(int i = 0; i < j; i++){
      vsum += max(v[i], 0);
    }
    for(int i = 0; i < l - j; i++){
      vsum += max(v[n - 1 - i], 0);
    }
    ans = max(ans, vsum);
  }
  

  
  for(l = k / 2 + 1; (l <= k && l <= n); l++){
    for(int j = 0; j <= l; j++){
      long long minus[50] = {};
      int c = 0;
      vsum = 0;
      for(int i = 0; i < j; i++){
	if(v[i] < 0) minus[c++] = v[i];
	vsum += v[i];
      }
      for(int i = 0; i < l - j; i++){
	if(v[n - 1 - i] < 0) minus[c++] = v[n - 1 - i];
	vsum += v[n - 1 - i];
      }
      qsort(minus, c, sizeof(long long), dsc);
      int e = 0;
      while(e < k - l && e < c){
	vsum -= minus[e++];
      }
      ans = max(ans, vsum);
    }
  }

  printf("%lld\n", ans);

  return 0;
}
      
      
		       

  

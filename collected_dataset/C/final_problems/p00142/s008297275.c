#include <stdio.h>
#include <string.h>
int main(){
  int exp[10001], n, i, j, mod[10001], a[10001], c, count[6000], h, d;

  for(i = 1; i < 10001; ++i) exp[i] = i * i;

  while(scanf("%d", &n) && n){
    memset(count , 0, sizeof(count));
    for(i = 1; i < n; ++i) mod[i] = exp[i] % n;
    c = 0;
    for(i = 1; i < n; ++i){
      for(j = 0; j < c; ++j)
	if(mod[i] == a[j]) break;
      if(j == c) a[c++] = mod[i];
    }
    h = (n - 1) / 2;
    for(i = 0; i < c; ++i){
      for(j = 0; j < c; ++j){
	if(a[i] != a[j]){
	  d = a[i] - a[j];
	  if(d < 0) d += n;
	  if(d > h) d = n - d;
	  count[d]++;
	}
      }
    }

    for(i = 1; i <= h; ++i) printf("%d\n", count[i]);  
  }
  return 0;
}
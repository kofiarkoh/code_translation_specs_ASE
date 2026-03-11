#include <stdio.h>

int main(){
  int w[11], s[11], i, j, n, wmax, res, sum;
  char f[11][21];

  while(scanf("%d", &n) && n){
    sum = 0;
    for(i = 0; i < n; ++i){
      scanf("%s %d %d", f[i], &w[i], &s[i]);
      sum += w[i];
    }

    for(j = 0; j < n; ++j){
      wmax = 0;
      for(i = 0; i < n; ++i)
	if(sum - w[i] <= s[i] && wmax <= w[i])
	  res = i, wmax = w[i];
      
      puts(f[res]);
      s[res] = -999999;
      sum -= w[res];
    }
  }
  return 0;
}
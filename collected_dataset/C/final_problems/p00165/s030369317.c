#include <stdio.h>

#define MAX 10000000

int pp[MAX + 1];

int main(){
  int n, p, m, i, j, prime, x, res, c;
  pp[0] = pp[1] = 1;
  for(i = 2; i < MAX / 2 ; ++i)
    if(pp[i] != 1)
      for(j = i * 2; j < MAX; j += i)
	pp[j] = 1;

  while(scanf("%d", &n) && n){
    res = 0;
    while(n--){
      scanf("%d %d", &p, &m);
      c = 0;
      for(i = ((p - m) < 0 ? 0 : p - m);i < ((p + m) > MAX ? MAX : p + m); ++i)
	if(pp[i] == 0) c++;
      if(c > 0) c--;
      res += c;
    }
    printf("%d\n", res);
  }
  return 0;
}
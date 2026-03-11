#include <stdio.h>

int main(){
  int n, p[31] = {}, i, res;
  p[0] = 1;
  p[1] = 1;
  p[2] = 2;


  while(scanf("%d", &n) && n){
    for(i = 3; i <= n; i++) p[i] = p[i - 1] + p[i - 2] + p[i - 3];
    res = p[n] / 10 / 365 + 1;
    printf("%d\n", res);
  }
  return 0;
}
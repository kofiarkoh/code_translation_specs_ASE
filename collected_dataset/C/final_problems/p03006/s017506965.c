#include "stdio.h"

int main(){

  int N, a, b, c, d, e, f = 0, max = 0;
  double x[50] = {}, y[50] = {};
  scanf("%d", &N);
  for(a = 0; a < N; a++){
    scanf("%d %d", &x[a], &y[a]);
  }
  for(b = 0; b < N; b++){
    for(c = b + 1; c < N; c++){
      f = 0;
      for(d = 0; d < N; d++){
        for(e = d + 1 ; e < N; e++){
          if((x[b] - x[c]) == (x[d] - x[e]) && (y[b] -
            y[c]) == (y[d] - y[e])){
              f++;
          }
        }
      }
      if(f > max){
        max = f;
      }
    }
  }
  printf("%d", N - max);
}

#include <stdio.h>


int main(){

  int n, x, y, w, h, N, i, j, cx, cy, count;

  scanf("%d", &n);
  for(i=0;i<n;i++){
    count = 0;

    scanf("%d %d %d %d", &x, &y, &w, &h);
    scanf("%d", &N);

    for(j=0;j<N;j++){
      
      scanf("%d %d", &cx, &cy);

      if(cx>=x && cx<=x+w && cy>=y && cy<=y+h) count++;

    }
    
    printf("%d\n", count);

  }

  
  return 0;
}
#include <stdio.h>
#include <stdlib.h>

int main(void){
  int i, n, *R, min, max;

  scanf("%d", &n);
  R = (int *)malloc(sizeof(int) * n);
  if(R == NULL){
    printf("?????¢????¢??????¨??????\n");
    return -1;
  }
  for(i = 0; i < n; i++){
    scanf("%d", &R[i]);
  }
  min = R[0];
  max = R[1] - R[0];
  for(i = 2; i < n; i++){
    if(max < R[i] - min){
      max = R[i] - min;
    }
    if(min > R[i]){
      min = R[i];
    }
  }
  printf("%d\n", max);
  free(R);
  return 0;
}
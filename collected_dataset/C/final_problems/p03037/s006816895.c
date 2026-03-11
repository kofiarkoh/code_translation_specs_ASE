#include<stdio.h>

int main(){

  int i, N, M, L, R, count=0, max_L, min_R;
  
  scanf("%d %d", &N, &M);
  scanf("%d %d", &L, &R);

  max_L = L;
  min_R = R;
  for (i = 0; i < M-1; i++){
	  scanf("%d %d", &L, &R);
	  if (L>max_L)
		  max_L=L;
	  if (R<min_R)
		  min_R=R;
  }
  count = min_R - max_L+1;
  if (count < 0)
	  count = 0;
  printf("%d\n", count);

}
    
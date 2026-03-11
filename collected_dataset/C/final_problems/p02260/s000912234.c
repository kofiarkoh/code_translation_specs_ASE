#include <stdio.h>
#include <stdlib.h>
int main(){
  int *A, N,i, j, minj, temp, count;

  scanf("%d",&N);
  A = (int *)malloc(sizeof(int) * N);
  for(j=0; j<N; j++){
    scanf("%d",&A[j]);
  }

  count = 0;
  for(i=0; i<N; i++){
    minj = i;
    for(j=i; j<N; j++){
      if(A[j]<A[minj]){
	minj = j;
      }
    }
    if(A[i] != A[minj]){
	temp = A[i];
	A[i] = A[minj];
	A[minj] = temp;
	count++;
      }
  }
  
  for(j=0; j<N; j++){
    printf("%d",A[j]);
    if(j != N-1) printf(" ");
  }
  printf("\n%d\n",count);
  
  return 0;
}


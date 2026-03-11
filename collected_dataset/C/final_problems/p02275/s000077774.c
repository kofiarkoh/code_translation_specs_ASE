#include <stdio.h>
#include <stdlib.h>
#define MAX 2000001
#define VMAX 10000

int main(){
  unsigned short *A, *B;
  int C[VMAX+1];
  int n, i, j,k;

  scanf("%d", &n);

  A = malloc(sizeof(short)*n+1);
  B = malloc(sizeof(short)*n+1);

  /* your code */
  k=-1;
  for(i=0; i<n; i++){
      scanf("%d",&A[i]);
      if(A[i]>k){
          k=A[i];
      }
  }
  for(i=0; i<=k; i++){
      C[i]=0;
  }
  for(i=0; i<=n; i++){
      C[A[i]]++;
  }
  for(i=1; i<=k; i++){
      C[i]+=C[i-1];
  }
  for(i=n; i>=0; i--){
      B[C[A[i]]]=A[i];
      C[A[i]]--;
  }

  printf("%d",B[2]);
  for(i=3; i<n+2; i++){
      if(i)printf(" ");
      printf("%d",B[i]);
  }
  printf("\n");

  return 0;
}

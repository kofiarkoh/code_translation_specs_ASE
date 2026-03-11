#include <stdio.h>
int main(){
  int count = 0;
  int N;
  int i,j;
  scanf("%d\n",&N);
  int A[N];
  int B[N];
  for(i=0;i<N;i++){
    scanf("%d %d\n",&A[i],&B[i]);
  }

  for(i = N-1;i>=0;i--){
    while( A[i]%B[i] != 0){
      for(j=0;j<=i;j++){
        A[j]++;
      }
      count++;
    }
  }
  printf("%d\n",count);
  return 0;
}

#include <stdio.h>
int main() {
  int N,i,max=0;
  scanf("%d", &N);
  int A[N+1],B[N];
  for(i=0;i<N+1;i++){
    scanf("%d", &A[i]);
  }
  for(i=0;i<N;i++){
    scanf("%d", &B[i]);
  }
  for(i=N-1;i>=0;i--){
    if(B[i]>=A[i+1]){
      max+=A[i+1];
      B[i]-=A[i+1];
      A[i+1]=0;
    }
    else{
      max+=B[i];
      B[i]=0;
      A[i+1]-=B[i];
    }

    if(B[i]>=A[i]){
      max+=A[i];
      B[i]-=A[i];
      A[i]=0;
    }
    else{
      max+=B[i];
      B[i]=0;
      A[i]-=B[i];
    }
    
  }
  
  printf("%d\n",max);
  
  return 0;
}

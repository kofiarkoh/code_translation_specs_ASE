#include <stdio.h>
#include <stdlib.h>
 
int comp(const void *a, const void *b){
    return *(int*)a-*(int*)b;
}

int main(void){
  int N,r;
  scanf("%d",&N);
  int A[N+1];
  for(int i=0;i<N;i++){
    scanf("%d",&A[i]);
  }
  qsort(A,N,sizeof(int),comp);
  while(A[0]!=A[N-1]){
    r=A[0];
    for(int i=1;i<N;i++){
      if(A[i]%A[0]==0){
        A[i]=A[0];
      }
      else{
        A[i]=A[i]%A[0];
      }
    }
    qsort(A,N,sizeof(int),comp);
  }
  printf("%d",r);
  return 0;
}


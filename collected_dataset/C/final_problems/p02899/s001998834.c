#include <stdio.h>
#include <string.h>
#include <math.h>

int main(void){
  long i;
  long N;
  long t;
  scanf("%ld",&N);
  long A[N+1];

  for(i=1; i<=N; i++){
    scanf("%ld",&t);
    A[t]=i;
  }

  for(i=1; i<=N; i++){
    if(i!=N){
      printf("%ld ",A[i]);
    }else{
      printf("%ld\n",A[i]);
    }
  }

  return 0;
}

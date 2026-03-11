#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int main(void){
  long int N,Q;
  long int K;
  long int *score;
  long int *A;
  scanf("%ld %ld %ld",&N,&K,&Q);
  score = calloc(N, sizeof(long int));
  A = calloc(Q, sizeof(long int));
  long int i;

  for (i = 1; i <= Q; i++) {
    scanf("%ld", &A[i]);
  }

  for (i = 1; i <= Q; i++) {
    score[A[i]] += 1;
  }

  for (i = 1; i <= N; i++) {
    if (K - Q + score[i] > 0) {
      puts("Yes");
    }else{
      puts("No");
    }
  }

  return 0;
}

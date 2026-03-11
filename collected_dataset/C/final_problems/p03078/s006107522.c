#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int uint64_sort( const void * a , const void * b ) {
  if(*(uint64_t*)b == *(uint64_t*)a) return 0;
  else if(*(uint64_t*)b >= *(uint64_t*)a) return 1;
  else return -1;
}

int main(void) {
  int X, Y, Z, K;
  scanf("%d %d %d %d", &X, &Y, &Z, &K);
  uint64_t A[1000];
  uint64_t B[1000];
  uint64_t C[1000];
  for(int i = 0; i < X; i++)
    scanf("%ld", &A[i]);
  for(int i = 0; i < Y; i++)
    scanf("%ld", &B[i]);
  for(int i = 0; i < Z; i++)
    scanf("%ld", &C[i]);
  qsort((void*)A, X, sizeof(uint64_t), uint64_sort);
  qsort((void*)B, Y, sizeof(uint64_t), uint64_sort);
  qsort((void*)C, Z, sizeof(uint64_t), uint64_sort);

  uint64_t ANS[3000];
  int idx = 0;
  for(int i = 0; i < X; i++) {
    for(int j = 0; j < Y; j++) {
      for(int k = 0; k < Z; k++) {
	if((i+1)*(j+1)*(k+1) <= K)
	  ANS[idx++] = A[i] + B[j] + C[k];
	else
	  break;
      }
    }
  }

  qsort((void*)ANS, idx, sizeof(uint64_t), uint64_sort);
  for(int i = 0; i < K; i++) {
    printf("%ld\n", ANS[i]);
  }
  
  return 0;
}

#include<stdio.h>
#include<stdlib.h>

int comp(const void *a, const void *b){
  return -1*(*(int*)a - *(int*)b);
}

int main(){
  int N;
  scanf("%d", &N);

  int A[N];
  int B[N];

  int C[N];
  long sumA = 0;
  long sumB = 0;
  int i;
  int M = 0;
  int m = 0;
  int sum = 0;
  int sum_m = 0;

  for(i = 0; i < N; i++){
    scanf("%d", &A[i]);
    sumA += A[i];
  }

  for(i = 0; i < N; i++){
    scanf("%d", &B[i]);
    sumB += B[i];
    C[i] = A[i] - B[i];
    if(C[i] < 0){
      m++;
      sum_m += C[i];
    }
  }
  sum_m *= -1;

  if(sumA < sumB){
    printf("-1\n");
    return 0;
  }
  if(m == 0){
    printf("0\n");
    return 0;
  }

  qsort(C, N, sizeof(int), comp);
  for(i = 0; i < N; i++){
    sum += C[i];
    if(sum >= sum_m) break;
  }

  M = m + i + 1;

  printf("%d\n", M);
  return 0;
}
  
  
  
  
  
  
  
  
  
  
  
  
  
  
    

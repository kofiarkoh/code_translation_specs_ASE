#include <stdio.h>
#include <stdlib.h>

int main()
{
  int N; 
  long long int answer;
  int *A, *B;
  scanf("%d", &N);
  answer = 0;
  A = (int *)malloc(sizeof(int) * N);
  B = (int *)malloc(sizeof(int) * N);
  for (int i = 0; i < N; i++) {
    scanf("%d", &A[i]);
    B[i] = A[i];
  }
  
  for (int i = 0; i < N - 1; i++) {
    if(B[i + 1] < B[i])
      B[i + 1] = B[i];
  }
  
  for(int i = 0; i < N; i++)
    answer += B[i] - A[i];
  printf("%lld\n", answer);
  return 0;
}
  
#include <stdio.h>

int main() {
  int N;
  int A[21];
  int B[21];
  int C[21];
  int tmp=0;
  int j = 0;
  int S = 0;
  scanf("%d",&N);
  for (int i = 0; i < N; i++) {
    scanf("%d",&A[i]);
  }
  for (int i = 0; i < N; i++) {
    scanf("%d",&B[i]);
  }
  for (int i = 0; i < N-1; i++) {
    scanf("%d",&C[i]);
  }
  for (int i = 1; i < N; i++) {
    if (A[i] == A[i-1]+1) {
      j = A[i];
      tmp = tmp + C[j-2];
    }
  }
  for (int i = 0; i < N; i++) {
    S = S+B[i];
  }
/*  printf("%d\n",S);
  printf("%d\n",tmp);*/
  printf("%d\n",S+tmp);
  return 0;
}

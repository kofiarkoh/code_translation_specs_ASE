#include <stdio.h>

int A[100000];

int GCD(int N, int m, int c){
  int i, j;
  if(m % 2 != c) m--; 
  for(i = m;i > 1;i -= 2){
    if(m % i != 0) continue;
    for(j = 0;j < N;j++) if(A[j] % i != 0) break;
    if(j == N) return i;
  }
  return 1;
}

void Do(int N){
  int i, m, c;
  m = 1e9;
  c = 0;
  for(i = 0;i < N;i++){
    scanf("%d", &A[i]);
    if(A[i] % 2 != 0) c = 1;
    if(m > A[i]) m = A[i];
  }
  printf("%d\n", GCD(N, m, c));
}

int main(void){
  int N;
  scanf("%d", &N);
  Do(N);
  return 0;
}
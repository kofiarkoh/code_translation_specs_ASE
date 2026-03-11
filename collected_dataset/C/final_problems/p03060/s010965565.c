#include<stdio.h>

int main(){
  int N;
  scanf("%d", &N);
  int V[N];
  int C[N];
  for(int i = 0; i < N; i++){
    scanf("%d", &V[i]);
  }
  
  for(int i = 0; i < N; i++){
    scanf("%d", &C[i]);
  }
  
  int X_Y = 0;
  for(int i = 0; i < N; i++){
    if(V[i] > C[i])
      X_Y += V[i] - C[i];
  }
  
  printf("%d\n", X_Y);
  
  return 0;
}
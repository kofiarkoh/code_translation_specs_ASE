#include<stdio.h>

const int MAX = 15000;

int abs(int x){
  return x > 0 ? x : -x;
}

int main(){
  int H, W;
  scanf("%d %d", &H, &W);
  
  int A[H][W];
  int B[H][W];
  for(int i = 0; i < H; i++)
    for(int j = 0; j < W; j++)
      scanf("%d", &A[i][j]);
  
  for(int i = 0; i < H; i++)
    for(int j = 0; j < W; j++)
      scanf("%d", &B[i][j]);
  
  int dp[H][W][MAX + 1];
  for(int i = 0; i < H; i++)
    for(int j = 0; j < W; j++)
      for(int k = 0; k <= MAX; k++)
        dp[i][j][k] = 0;
  
  dp[0][0][abs(A[0][0] - B[0][0])] = 1;
  
  for(int i = 0; i < H; i++){
    for(int j = 0; j < W; j++){
      for(int k = 0; k <= MAX; k++){
        if(dp[i][j][k] == 0) continue;
        if(i < H - 1){
          dp[i + 1][j][k + abs(A[i + 1][j] - B[i + 1][j])] = 1;
          dp[i + 1][j][abs(k - abs(A[i + 1][j] - B[i + 1][j]))] = 1;
        }
        if(j < W - 1){
          dp[i][j + 1][k + abs(A[i][j + 1] - B[i][j + 1])] = 1;
          dp[i][j + 1][abs(k - abs(A[i][j + 1] - B[i][j + 1]))] = 1;
        }
      }
    }
  }
  
  int b = 0;
  for(int k = 0; k <= MAX; k++){
    if(dp[H - 1][W - 1][k]){
      b = k;
      break;
    }
  }
  
  printf("%d\n", b);
  
  return 0;
}
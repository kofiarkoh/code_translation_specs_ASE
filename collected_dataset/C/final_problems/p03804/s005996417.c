#include<stdio.h>
int main(void)
{
  int N, M;
  char A[51][51];
  char B[51][51];

  scanf("%d %d", &N, &M);
  for(int i = 0; i < N; i++){
    scanf("%s", A[i]);
  }
  for(int i = 0; i < M; i++){
    scanf("%s", B[i]);
  }
  for(int i = 0; i < N; i++){
    for(int j = 0; j < N; j++){
      int isOK = 1;
      for(int k = 0; k < M; k++){
        for (int l = 0; l < M; l++) {
          if (i + k >= N) {
            isOK = 0;
            break;
          }
          if (j + l >= N) {
            isOK = 0;
            break;
          }
          if (A[i + k][j + l] != B[k][l]) {
            isOK = 0;
          }
        }
      }
      if (isOK) {
        printf("Yes\n");
        return 0;
      }
    }
  }

  printf("No\n");
  return 0;
}


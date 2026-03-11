#include <stdio.h>
int main(void)
{
  int N,X;
  int L_1[100];
  int i, j;
  int sum = 0;
  int result = 0;

  scanf("%d", &N);
  scanf("%d", &X);

  for(i = 0; i < N; i++){
    scanf("%d", &L_1[i]);
  }

  for(j = 0; sum <= X; j++){
    sum += L_1[j];
    result++;
  }


  printf("%d", result);

  return(0);
}
#include <stdio.h>

int main(void){
  int A, B, i, count = 0, digit_A[5], digit_B[5];
  scanf("%d %d", &A, &B);
  for(i=0; i<5; i++){
    digit_A[i] = A % 10;
    A = A / 10;
    digit_B[i] = B % 10;
    B = B / 10;
  }
  count = (digit_B[4] - digit_A[4]) * 100 + 
          (digit_B[3] - digit_A[3] - 1) * 10 +
          9 - digit_A[2] + digit_B[2];
  if(digit_A[1] * 10 + digit_A[0] <= digit_A[3] * 10 + digit_A[4])
    count++;
  if(digit_B[1] * 10 + digit_B[0] >= digit_B[3] * 10 + digit_B[4])
    count++;
  printf("%d",count);
  return 0;
}
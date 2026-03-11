#include <stdio.h>
int main(){
  int A;
  scanf("%d", &A);
  int D = A % 10;
  int C = A % 100 / 10;
  int B = A % 1000 / 100;
  A /= 1000;
  if(A + B + C + D == 7) printf("%d+%d+%d+%d=7\n", A, B, C, D);
  else if(A + B + C - D == 7) printf("%d+%d+%d-%d=7\n", A, B, C, D);
  else if(A + B - C + D == 7) printf("%d+%d-%d+%d=7\n", A, B, C, D);
  else if(A - B + C + D == 7) printf("%d-%d+%d+%d=7\n", A, B, C, D);
  else if(A + B - C - D == 7) printf("%d+%d-%d-%d=7\n", A, B, C, D);
  else if(A - B + C - D == 7) printf("%d-%d+%d-%d=7\n", A, B, C, D);
  else if(A - B - C + D == 7) printf("%d-%d-%d+%d=7\n", A, B, C, D);
  else if(A - B - C - D == 7) printf("%d-%d-%d-%d=7\n", A, B, C, D);
}
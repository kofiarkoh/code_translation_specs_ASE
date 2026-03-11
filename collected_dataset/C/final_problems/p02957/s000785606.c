#include <stdio.h>
int main(void)
{
  int A, B, K;
  scanf("%d %d", &A, &B);
  if(A >= B)
    K = B + (A - B) / 2;
  else
    K = A + (B - A) / 2;
  
  if(A % 2 == B % 2)
    printf("%d", K);
  else
    printf("IMPOSSIBLE");

  return 0;
}

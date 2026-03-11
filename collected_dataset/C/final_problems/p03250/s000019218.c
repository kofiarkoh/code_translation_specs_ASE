#include<stdio.h>

int main()
{
  int A, B, C, chan, r;
  scanf("%d %d %d", &A, &B, &C);
  
  if (B > C)
  {
    chan = B;
      B = C;
      C = chan;
  }
  
  if (A > B)
  {
    chan = A;
      A = B;
      B = chan;
  }
  
   if (A > C)
  {
    chan = A;
      A = C;
      C = chan;
  }
  
  printf("%d", 10*C + A +B);
  
  return 0;
}
/* ????????? */

#include <stdio.h>

int main(void) {
  unsigned long  N, A, B, AandB;
  unsigned long  count = 0;
  unsigned long  i = 0;
  scanf("%lu%lu%lu",&N, &A, &B);
  
  AandB = A+B;
  /* ????????????????????????????? */
  if(N/10000000000000 >= AandB){
    while(N/(i+=10000000000000) >= (AandB));
  	i -= 10000000000000;
  }
  /* !!!!!!!!????????!!!!?!?!?!?!?!? */
  if(N/10000000 >= AandB){
    while(N/(i+=10000000) >= (AandB));
  	i -= 10000000;
  }
  while(N/(++i) >= (AandB));
  
  N -= (AandB)*--i;
  count = A*i;
  
  if(N < A)
    count += N;
  else
    count += A;
  printf("%lu\n",count);
  
  return 0;
}
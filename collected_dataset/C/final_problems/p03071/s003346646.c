#include <stdio.h>

int main()
{
  int A,B;
  scanf("%d %d",&A,&B);
  if (A+1<B)
  {
    printf("%d",2*B-1);
  }
  else if (B+1<A)
    printf("%d",2*A-1);
  else
  	printf("%d",A+B);
}
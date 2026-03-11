#include<stdio.h>

int main(){
  int X,A,B;
  scanf("%d",&X);

  for (int i = 1; i <= X; i++)
  {
    A=i*(i-1)/2;
    B=i*(i+1)/2;
    if (A<=X&&X<=B)
    {
      printf("%d",i);
      break;
    }
  }

  return 0;
}
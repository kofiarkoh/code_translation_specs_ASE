#include<stdio.h>
int main(void){
  int a,b;
  scanf("%d %d", &a, &b);
  if((a*b)%2) printf("Odd\n");
  else printf("Even\n");
  return 0;
}
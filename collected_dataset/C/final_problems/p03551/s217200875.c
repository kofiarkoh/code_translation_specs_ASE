#include<stdio.h>

int main(void){
  int N,M;
  scanf("%d%d",&N,&M);
  printf("%d\n",(100*(N-M)+1900*M)<<M);
  return 0;
}

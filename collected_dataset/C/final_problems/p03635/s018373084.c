#include<stdio.h>

int main(void){
  int n=0,m=0;
  int N=0,M=0;
  scanf("%d %d",&n,&m);
  N = n - 1;
  M = m - 1;
  
  printf("%d", N*M);
  
  return 0;
}
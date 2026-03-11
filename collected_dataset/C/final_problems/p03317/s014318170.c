#include<stdio.h>
#include<math.h>
#include<stdlib.h>

int main(void){
  int N, K;
  scanf("%d%d",&N,&K);
  int i,j;
  int AMARI = N / K;
	while(N % K != 0)
 	 N = N + AMARI;
 
  printf("%d",N/K);

  return 0;
}
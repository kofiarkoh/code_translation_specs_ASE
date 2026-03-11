#include<stdio.h>

int main(void){
  int n,i=1,N=1,yakusuu;
  scanf("%d",&n);

  while(1){
    yakusuu=0;
    while(i<=N){
        if(N%i==0) yakusuu++;
        i++;
    }
    if(yakusuu==n) break;
    i=1;  N++;
  }
  printf("%d\n",N);
  return 0;
}


#include<stdio.h>
#include<stdlib.h>
int main(void){
  int N,sum=0,min=0,x=0;
  int W[100];
  scanf("%d",&N);
  for(int i=0;i<N;i++){
    scanf("%d",&W[i]);
    sum+=W[i];
  }
  for(int j=0;j<N-1;j++){
    x+=W[j];
    sum-=W[j];
    if(j==0 || abs(sum-x)<min){
      min=abs(sum-x);
    }
  }
  printf("%d\n",min);
  return 0;
}
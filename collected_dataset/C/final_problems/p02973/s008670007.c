#include<stdio.h>
int main(void){
  int N,i,j,max=1,count=1;
  scanf("%d",&N);
  int A[N];
  for(i=0;i<N;i++){
    scanf("%d",&A[i]);
  }
  for(i=0;i<N-1;i++){
    if(A[i]>=A[i+1]){
      count++;
    }
    else{
      count=1;
    }
    if(max<count){
      max=count;
    }
  }
  printf("%d",max);
  return 0;
}